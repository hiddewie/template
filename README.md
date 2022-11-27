# template

CLI for templating based on JSON, YAML or HCL configuration

## Installation

Compile with
```shell
cargo build --release
```

## Usage

Run the `template` CLI with
```shell
target/release/template path/to/template.template path/to/configuration.json
```

The output is rendered to the standard output stream. Log messages are output to the standard error stream.

### Exit codes

- `0`: Success. The standard output contains the rendered template.
- `1`: Template file cannot be read.
- `2`: Configuration file cannot be read.
- `3`: Configuration file cannot be parsed.
- `4`: Template file cannot be parsed.
- `5`: Template cannot be rendered.
- `101`: Panic. An unexpected error has occurred, and was not handled correctly. Please [create an issue](https://github.com/hiddewie/template/issues) to report the configuration, the template and the error output.

## Configuration

- JSON

## Templating

See [Pest grammar](./src/template.pest) for formal template grammar.

Extension recommended `.template`, but not enforced.

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

### Flow control

Test expressions with `if`, `elif` and `else`:
```
{% if some_expression %}
  Rendered when the expression is truthy
{% elif else_if_expression %}
  Rendered when the above is not true and the expression is truthy
{% else %}
  Rendered when the above are not truthy
{% end %}
```

Loop over arrays with `for` and `else`:
```
{% for item in array_value %}
  Rendered content for each item
  {% item.property %} Propreties can be referenced
{% else %}
  Rendered when the array did not contain any values
{% end %}
```

### Functions

#### String functions

- `upperCase`: transform a string into upper case.
- `lowerCase`: transform a string into lower case.
- `kebabCase`: transform a string into kebab case: `lowercase-words-joined-with-dashes`.
- `snakeCase`: transform a string into snake case: `lowercase_words_joined_with_underscores`.
- `camelCase`: transform a string into camel case: `capitalizedWordsWithoutSpaces`.
- `pascalCase`: transform a string into pascal case: `CapitalizedWordsWithoutSpaces`.
- `capitalize`: make the first character uppercase.
- `capitalizeWords`: make the first character of every word uppercase.
- `environment`: read an environment variable.

## Development

### Build

```shell
cargo build
```

### Test

```shell
cargo test
```