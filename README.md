# Template ![crates.io](https://img.shields.io/crates/v/template-cli.svg)

CLI for templating based on JSON, YAML or HCL configuration.

## Installation

Install with
```shell
cargo install template-cli
```

## Usage

```shell
template --template ./path/to/template.template --configuration ./path/to/configuration.json
```

Show the usage information with the `--help` option:
```shell
template --help
```
which outputs:
```text
CLI for templating based on JSON, YAML or HCL configuration

Usage: template [OPTIONS] --template <TEMPLATE> --configuration <CONFIGURATION>

Options:
  -t, --template <TEMPLATE>            Absolute or relative path to the template file
  -c, --configuration <CONFIGURATION>  Absolute or relative path to the configuration file. Provide `-` as path to read the configuration input from the standard input stream
  -f, --format <FORMAT>                Specify the format of the configuration input. Useful when the configuration file has a non-standard extension, or when the input is given in the standard input stream [possible values: json, hcl, yaml]
  -h, --help                           Print help information
  -V, --version                        Print version information
```

The output is rendered to the standard output stream. Log messages are output to the standard error stream.

This tool combines well with other command-line utilities, following the UNIX philosophy, for example:
```shell
curl -s 'https://dummyjson.com/products' \
  | jq '{ items: .products }' \
  | template --configuration - --template products.template \
  | head -n 5 \
  > product-list.txt
```

### Exit codes

- `0`: Success. The standard output contains the rendered template.
- `1`: Template file cannot be read.
- `2`: One of the required CLI options is not given.
- `3`: Configuration file cannot be read.
- `4`: Configuration file cannot be parsed.
- `5`: Template file cannot be parsed.
- `6`: Template cannot be rendered.
- `101`: Panic. An unexpected error has occurred, and was not handled correctly. Please [create an issue](https://github.com/hiddewie/template/issues) to report the configuration, the template and the error output.

### Security

This software:
- reads the configuration file or standard input
- reads the template file
- reads the environment, if the `environment` function is used
- does not write any files
- does not make any network connections

## Configuration

- [JSON](https://www.json.org), extension `.json`, default parsing format
- [HCL](https://github.com/hashicorp/hcl), extension `.hcl`
- [YAML](https://yaml.org/spec/), extension `.yml` and `.yaml`

If the configuration file has a different extension, or when the configuration is given in the standard input stream, you can configure the configuration format with the `--format` option.

## Templating

See [Pest grammar](./src/template.pest) for formal template grammar.

It is recommended to use the extension `.template` for template files.

### Render configuration data

Configuration 
```json
{
  "value": "Hello",
  "nested": {
    "value": "world"
  }
}
```
with template
```
{% value %} {% nested.value %}{% does_not_exist %}!
```
renders output
```
Hello world!
```

### Comments

Comments are ignored and the content will not be rendered as output

```
This is rendered {# comment #} 
{# {% for item in array %}
This is not a loop
{% end %} #}
This is rendered
```

### Literals

```
Null: {% null %}
Boolean: {% true %} and {% false %}
Integer: {% 0 %}, {% -0 %}, {% -100 %} and {% 100 %}
Floating point: {% .0 %}, {% 0. %}, {% -1.0 %}, {% 10.47e1 %} and {% -1.47e-10 %}
String: {% "" %} and {% "value" %}
Array: {% [] %}, {% [1] %} and {% ["", null, expression, [], {}, ] %}
Dictionary: {% {} %}, {% {a:1} %} and {% {item: expression, " space ": "spacy", "array": [], } %}
```

### Flow control

Test expressions with `if` / `unless`, `elif` and `else`:
```
{% if some_expression %}
  Rendered when the expression is truthy
{% elif else_if_expression %}
  Rendered when the above is not true and the expression is truthy
{% else %}
  Rendered when the above are not truthy
{% end %}

{% unless expression %}
  Rendered when the expression is falsy
{% end %}
```

Loop over arrays with `for` and `else`:
```
{% for item in array_value %}
  Rendered content for each item
  {% item.property %} Properties can be referenced
{% else %}
  Rendered when the array did not contain any values
{% end %}
```

### Functions

Apply a function in a template by using the pipe `|` operator:
```
{% value | function1 | function2 | function3 %}
```

#### General functions

- `default(value)`: default value if the argument is falsy.
- `toString`: transform the value to a string.

#### String functions

- `length`: length of the string.
- `upperCase`: transform a string into upper case.
- `lowerCase`: transform a string into lower case.
- `kebabCase`: transform a string into kebab case: `lowercase-words-joined-with-dashes`.
- `snakeCase`: transform a string into snake case: `lowercase_words_joined_with_underscores`.
- `camelCase`: transform a string into camel case: `capitalizedWordsWithoutSpaces`.
- `pascalCase`: transform a string into pascal case: `CapitalizedWordsWithoutSpaces`.
- `capitalize`: make the first character uppercase.
- `capitalizeWords`: make the first character of every word uppercase.
- `environment`: read an environment variable.
- `reverse`: the string in reverse order.
- `split(splitter)`: split the string for each occurrence of `splitter`.
- `parseBoolean`: parses a boolean.
- `parseInteger`: parses an integer.
- `parseDecimal`: parses a decimal number.
- `parseNumber`: parses an integer or decimal number.
- `matches(regex)`: checks if a number matches a regular expression.
- `substring(from)`, `substring(from, to)`: creates a substring from the string. `from` is inclusive, `to` is exclusive.
- `take(n)`: takes the first `n` characters from the string.
- `drop(n)`: drops the first `n` characters from the string.

#### Array functions

- `length`: length of the array.
- `reverse`: the array in reverse order.
- `take(n)`: takes the first `n` items from the array.
- `drop(n)`: drops the first `n` items from the array.
- `first`: the first item from the array, if it exists.
- `last`: the last item from the array, if it exists.

#### Dictionary functions

- `length`: size of the dictionary

## Development

### Build

```shell
cargo build
```

### Test

```shell
cargo test
```
