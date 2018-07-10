# Get Player

Returns a specific player

* **URL**

  `/player/[id]`

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
    GET /player/1
    content-type: application/json
    x-api-key: [API_KEY]
    cache-control: no-cache
    
    ---

    HTTP/1.1 200
    status: 200
    content-type: application/json
    server: Rocket
    content-length: 34
    date: Tue, 10 Jul 2018 03:57:34 GMT
    {"id":1,"alias":"sample","decks":[]}
    ```
