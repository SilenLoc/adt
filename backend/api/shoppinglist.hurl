# get items
GET {{target}}/items

HTTP 200


[Asserts]
jsonpath "$[?(@.title == 'Milk')]" exists
jsonpath "$[?(@.title == 'Milk')].uuid" nth 0 == "ac18131a-c7b8-4bdc-95b5-e1fb6cad4576"

# create item
POST {{target}}/items
Content-Type: application/json
Accept: application/json
{
"title": "Pepperoni", "posted_by": "Rustacean"
}

HTTP 200


# get items
GET {{target}}/items

HTTP 200


[Asserts]
jsonpath "$[?(@.title == 'Pepperoni')]" exists