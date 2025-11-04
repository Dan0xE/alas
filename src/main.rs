use nu_plugin::{
    EngineInterface, EvaluatedCall, MsgPackSerializer, Plugin, PluginCommand, serve_plugin,
};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Type, Value};

struct AlasPlugin;

impl Plugin for AlasPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(AlasCommand)]
    }
}

struct AlasCommand;

impl PluginCommand for AlasCommand {
    type Plugin = AlasPlugin;

    fn name(&self) -> &str {
        "alas"
    }

    fn description(&self) -> &str {
        "Check if an alias exists for a command and remind the user"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("command", SyntaxShape::String, "The command to check")
            .required(
                "aliases",
                SyntaxShape::List(Box::new(SyntaxShape::Record(vec![]))),
                "List of aliases from scope",
            )
            .input_output_type(Type::Nothing, Type::String)
            .category(Category::Custom("alias".into()))
    }

    fn run(
        &self,
        _plugin: &AlasPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let command: String = call.req(0)?;
        let aliases_value: Value = call.req(1)?;

        let aliases = parse_aliases(&aliases_value)?;

        if let Some((alias_name, alias_value)) = find_best_alias(&command, &aliases) {
            let message = format!("Alias '{}' exists for '{}'", alias_name, alias_value);
            Ok(PipelineData::Value(Value::string(message, call.head), None))
        } else {
            // no alias found
            Ok(PipelineData::Value(Value::string("", call.head), None))
        }
    }
}

fn parse_aliases(aliases_value: &Value) -> Result<Vec<(String, String)>, LabeledError> {
    let mut aliases = Vec::new();

    if let Value::List { vals, .. } = aliases_value {
        for val in vals {
            if let Value::Record { val: record, .. } = val {
                let name = record
                    .get("name")
                    .and_then(|v| v.as_str().ok())
                    .map(|s| s.to_string());

                let expansion = if let Some(exp_val) = record.get("expansion") {
                    match exp_val {
                        Value::String { val, .. } => Some(val.clone()),
                        Value::List { vals, .. } => {
                            let parts: Result<Vec<String>, _> = vals
                                .iter()
                                .map(|v| v.as_str().map(|s| s.to_string()))
                                .collect();
                            parts.ok().map(|p| p.join(" "))
                        }
                        _ => None,
                    }
                } else {
                    None
                };

                if let (Some(name), Some(expansion)) = (name, expansion) {
                    aliases.push((name, expansion));
                }
            }
        }
    }

    Ok(aliases)
}

fn find_best_alias(command: &str, aliases: &[(String, String)]) -> Option<(String, String)> {
    let mut best_match: Option<(String, String)> = None;
    let mut best_match_len = 0;

    for (alias_name, alias_value) in aliases {
        if command == alias_value || command.starts_with(&format!("{} ", alias_value)) {
            // longer matches take precedence
            if alias_value.len() > best_match_len {
                best_match = Some((alias_name.clone(), alias_value.clone()));
                best_match_len = alias_value.len();
            }
        }
    }

    best_match
}

fn main() {
    serve_plugin(&AlasPlugin, MsgPackSerializer);
}
