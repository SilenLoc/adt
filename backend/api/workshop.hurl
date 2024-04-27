# wait for healthy
POST {{target}}/workshop
Content-Type: application/json
Accept: application/json
{
"attendees_count": 20, "people_like_it": true
}

HTTP 200


[Asserts]
jsonpath "$.attendees_count" == 20
jsonpath "$.people_like_it" == true