# Get Players

Returns a list of all available players

* **URL**

  `/players`

* **Method:**
  
  `GET`
  
*  **URL Params**

   NONE 

* **Data Params**

    NONE

* **Requires Authentication**

    YES

* **Success Response:**
  
  * **Code:** 200 <br />
    **Content:** `[{ "id": 1, "alias": "sample" }]`
 
* **Error Response:**

  Malformated JSON or an email or alias that already exist will result in an error.

  * **Code:** 401 Bad Request <br />
  * **Code:** 500 Internal Server Error <br />

* **Sample Call:**

    ```
    GET /players HTTP/1.1
    Host: localhost:8000
    Content-Type: application/json
    Cache-Control: no-cache

    ---

    HTTP/1.1 200
    status: 200
    content-type: application/json
    server: Rocket
    content-length: 28
    date: Sun, 08 Jul 2018 22:20:10 GMT
    [{"id":1,"alias":"sample2"}]
    ```
