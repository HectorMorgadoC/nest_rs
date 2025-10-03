curl -X POST -i
-H "Content-Type: application/json" -c coockie.txt
-d '{
"type": "auth",
"atributes": {
"email": "laury@soluciones.com",
"password": "12345678"
}
}'
http://127.0.0.1:3000/login

curl -X POST -i -H "Content-Type: application/json" -d '{ "type": "team", "atributes": {"name": "manzana","team": 24}}' http://127.0.0.1:3000/team
