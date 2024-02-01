## JSON Patch CLI

This is a simple CLI tool to apply a [JSON Patch](https://jsonpatch.com/) to a given JSON document based on the [idubrov/json-patch](https://github.com/idubrov/json-patch) JSON patch implementation.

## Example Usage
1. Pipe doc to json patch cli:
    ```shell
    echo '[
        { "name": "Andrew"},
        {"name": "Maxim"}
    ]' | json-patch-cli '[{ "op": "add", "path": "/0/happy", "value": true }]'
    # Outputs
    [{"happy":true,"name":"Andrew"},{"name":"Maxim"}]
    ```
1. Provide both doc and patch as arguments
    ```shell
     json-patch-cli '[{ "name": "Andrew" },{ "name": "Maxim" }]' '[{ "op": "add", "path": "/0/happy", "value": true }]' | jq
    ```

1. Reading a file for the JSON doc or patch input
    ```shell
    json-patch-cli doc.json '[{ "op": "add", "path": "/0/happy", "value": true }]' | jq
    ```