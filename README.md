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

- 0: Success. The standard output contains the rendered template.
- 101: Panic. An unexpected error has occurred which was not handled correctly. Please [create an issue](https://github.com/hiddewie/template/issues) to report the configuration, the template and the error output.

## Configuration

- JSON

## Templating

See [Pest grammar](./src/template.pest) for formal template grammar.

Extension recommended `.template`, but not enforced.

### Configuration values

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
{% value %}{% nested.value %}{% does_not_exist %}!
```
renders output
```
Hello world!
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

## Development

### Build

```shell
cargo build
```

### Test

```shell
cargo test
```