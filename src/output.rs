use chrono::{DateTime, Local};

/// Output formats supported by Sigi.
#[derive(Clone, Copy, PartialEq)]
pub enum OutputFormat {
    /// Comma-separated values.
    Csv,
    /// Human readable formats. Accepts a "noise level" for how much to output.
    Human(NoiseLevel),
    /// JSON (JavaScript Object Notation) - Pretty-printed with newlines and two-space indentation.
    Json,
    /// JSON (JavaScript Object Notation) - No newlines or indentation.
    JsonCompact,
    /// Print nothing at all.
    Silent,
    /// Tab-separated values.
    Tsv,
}

/// How much noise (verbosity) should be used when printing to standard output.
#[derive(Clone, Copy, PartialEq)]
pub enum NoiseLevel {
    Verbose,
    Normal,
    Quiet,
}

impl OutputFormat {
    pub fn format_time(&self, dt: DateTime<Local>) -> String {
        // TODO: This should be configurable.
        // TODO: Does this work for all locales?
        dt.to_rfc2822()
    }

    pub fn log(&self, labels: Vec<&str>, values: Vec<Vec<&str>>) {
        if let OutputFormat::Silent = self {
            return;
        }

        match &self {
            OutputFormat::Csv => {
                let print_csv = join_and_print(",");
                print_csv(labels);
                values.into_iter().for_each(print_csv)
            }
            OutputFormat::Human(noise) => match noise {
                NoiseLevel::Verbose => {
                    values.into_iter().for_each(|line| match line.len() {
                        0 => (),
                        1 => println!("{}", line.get(0).unwrap()),
                        2 => println!("{}: {}", line.get(0).unwrap(), line.get(1).unwrap()),
                        _ => println!(
                            "{}: {} ({})",
                            line.get(0).unwrap(),
                            line.get(1).unwrap(),
                            line.iter()
                                .skip(2)
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    });
                }
                NoiseLevel::Normal => {
                    // Print only first two values e.g. (num, item) separated by a single space.
                    values.into_iter().for_each(|line| {
                        if let (Some(label), Some(item)) = (line.get(0), line.get(1)) {
                            println!("{}: {}", label, item);
                        } else if let Some(info) = line.get(0) {
                            println!("{}", info);
                        }
                    });
                }
                NoiseLevel::Quiet => values.into_iter().for_each(|line| {
                    // Print only second value (item) separated by a single space.
                    if let Some(message) = line.get(1) {
                        println!("{}", message);
                    }
                }),
            },
            OutputFormat::Json => {
                let keys = labels;
                let objs = values
                    .into_iter()
                    .map(|vals| {
                        let mut obj = json::JsonValue::new_object();
                        keys.iter().zip(vals).for_each(|(k, v)| obj[*k] = v.into());
                        obj
                    })
                    .collect::<Vec<_>>();

                println!("{}", json::stringify_pretty(objs, 2));
            }
            OutputFormat::JsonCompact => {
                let keys = labels;
                let objs = values
                    .into_iter()
                    .map(|vals| {
                        let mut obj = json::JsonValue::new_object();
                        keys.iter().zip(vals).for_each(|(k, v)| obj[*k] = v.into());
                        obj
                    })
                    .collect::<Vec<_>>();

                println!("{}", json::stringify(objs));
            }
            OutputFormat::Silent => {
                unreachable!("[BUG] Sigi should always exit outputting before this point.")
            }
            OutputFormat::Tsv => {
                let print_tsv = join_and_print("\t");
                print_tsv(labels);
                values.into_iter().for_each(print_tsv)
            }
        }
    }
}

fn join_and_print(sep: &str) -> impl Fn(Vec<&str>) {
    let sep = sep.to_string();
    move |tokens: Vec<&str>| println!("{}", tokens.join(&sep))
}
