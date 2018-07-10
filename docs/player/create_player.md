# Create Player

Creates a new player account with a user name, email and password.

* **URL**

  `/player`

* **Method:**
  
  `POST`
  
*  **URL Params**

   NONE 

* **Data Params**

  ```
  {
    "alias": "sample",
    "email": "sample@example.com",
    "password": "123456789"
  }
  ```

* **Requires Authentication**

    YES

* **Success Response:**
  
  * **Code:** 200 <br />
    **Content:** `{ "id": 1, "alias": "sample" }`
 
* **Error Response:**

  Malformated JSON or an email or alias that already exist will result in an error.

  * **Code:** 401 Bad Request <br />
  * **Code:** 500 Internal Server Error <br />

* **Sample Call:**

    ```
    POST /player HTTP/1.1
    Host: localhost:8000
    Content-Type: application/json
    Cache-Control: no-cache

    {
        "alias": "sample",
        "email": "sample@example.com",
        "password": "123456789"
    }

    ---

    HTTP/1.1 200
    status: 200
    content-type: application/json
    server: Rocket
    content-length: 26
    date: Sun, 08 Jul 2018 22:05:20 GMT

    {"id":1,"alias":"sample"}
    ```
