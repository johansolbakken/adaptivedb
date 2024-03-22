#!/usr/bin/env python3

import requests
import json

def main():
    address="http://localhost:3000/schema"
    file_name="../queries/definition.ddl"

    with open(file_name, 'r') as file:
        schema = file.read()
        data = json.dumps({
            'schema': schema
        })

        headers = {
            'Content-Type': 'application/json'
        }
        
        response = requests.post(address, headers=headers, data=data)
        print(response.text)

if __name__ == "__main__":
    main()