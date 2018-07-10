# Login

This route allows a player to authenticate an account and recieve a JWT authentication token that should be sent with subsequent requests

* **URL**

  `/login`

* **Method:**
  
  `POST`
  
*  **URL Params**

   NONE 

* **Data Params**

  ```
  {
    "email": "sample@example.com",
    "password": "123456789"
  }
  ```

* **Requires Authentication**

    NO

* **Success Response:**
  
  * **Code:** 200 <br />
    **Content:** `{"token": "[auth token]"}`
 
* **Error Response:**

  Malformated JSON or an email or alias that already exist will result in an error.

  * **Code:** 401 Bad Request <br />
  * **Code:** 500 Internal Server Error <br />

* **Sample Call:**

    ```
    POST /login
    content-type: application/json
    cache-control: no-cache
    content-length: 62
    { 
        "email": "sample@example.com",
        "password": "123456789"
    }

    ---

    HTTP/1.1 200
    status: 200
    date: Tue, 10 Jul 2018 04:14:07 GMT
    server: Rocket
    content-type: application/json
    content-length: 129
    connection: Keep-Alive
    {"token":[authentication token]}
    ```
* **Notes**

  Tokens by default are valid for 7 days. After which the token will need to be refreshed or the player will have to login again to recieve a new token.

  Tokens should be included in the header of routes that require authentication with the key `x-api-token`.