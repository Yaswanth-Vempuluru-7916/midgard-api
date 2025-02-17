# Midgard Vault - API and Database Documentation

## Overview

The **Midgard Vault** project connects to the **Midgard API** to fetch historical data related to Depth History, Earnings History, Swaps History, and Rune Pool History. This data is stored in a MongoDB database, and custom API endpoints are provided for querying and interacting with the data.

The project is built using **Rust** and the **Axum** framework for handling HTTP requests. It is fully integrated with MongoDB to store large amounts of historical data, and a **background job** ensures that the database is kept up-to-date with the latest data from the Midgard API.

---

## Project Structure

```plaintext
📦 src
 ┣ 📂 api                      # API routes
 ┃ ┣ 📜 mod.rs                 # Registers all API endpoints
 ┃ ┣ 📜 depth_history.rs       # Endpoint: /api/depth-history
 ┃ ┣ 📜 earnings_history.rs    # Endpoint: /api/earnings-history
 ┃ ┣ 📜 swaps_history.rs       # Endpoint: /api/swaps-history
 ┃ ┣ 📜 runepool_history.rs    # Endpoint: /api/rune-pool-history
 ┣ 📂 config                   # Configuration files
 ┃ ┣ 📜 settings.rs
 ┃ ┣ 📜 mod.rs
 ┣ 📂 db                       # Database connection & models
 ┃ ┣ 📜 mongo.rs
 ┃ ┣ 📜 models.rs
 ┣ 📂 utils                    # Utility functions 
 ┃ ┣ 📜 conversion.rs          # Number conversion logic
 ┃ ┣ 📜 midgard_fetch.rs       # Fetching data from Midgard API
 ┣ 📜 main.rs                  # Main application entry point
 ┣ 📜 Cargo.toml
 ┣ 📜 .env                     # Environment variables
```

---

## 1. **Populating the Database Using the Midgard API**

### **Goal**:
Populate the MongoDB database with historical data from the **Midgard API**. The data includes:
- **Depth History** (data related to asset depth, LP units, and member count).
- **Earnings History** (data on earnings).
- **Swaps History** (historical data on swaps).
- **Rune Pool History** (data on Rune Pool activity).

The **Midgard API** provides these datasets in **hourly intervals**. This project fetches data from the Midgard API and stores it in MongoDB, ensuring that only **new records are inserted** and **duplicates are avoided** based on timestamps.

### **How It Works**:

- **API Fetching**: A background job is implemented to fetch data from the Midgard API every **hour**. The data is paginated and processed to avoid duplicate entries in the database.
- **Timestamp Handling**: The job uses the `endTime` field from the **Meta** data in the API response to determine where the new data should begin fetching from.
- **Data Insertion**: Once the data is fetched, it is inserted into the MongoDB collections: `depth_history`, `earnings_history`, `swaps_history`, and `rune_pool_history`.

---

### **Key Functions**:

- `fetch_and_store_data`: Main function to fetch data from the Midgard API for all data types (Depth History, Earnings History, etc.) and insert it into the MongoDB database.
- `get_last_stored_timestamp`: Retrieves the most recent `endTime` from MongoDB to ensure the fetch operation only retrieves new data.
- `fetch_paginated_data`: Handles the actual fetching of data from the API and storing it in the database, using pagination to retrieve all records.

#### Example Request:
```bash
GET https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&count=400&from={start_time}
```

---

## 2. **Scheduled Job for Hourly Updates**

The system automatically fetches new data from the Midgard API **every hour** to keep the database up-to-date with the latest information.

### **Job Logic**:

- The job runs every hour to fetch the latest data, starting from the most recent timestamp stored in the database.
- New data is only inserted if it doesn't already exist in the database, using the `endTime` timestamp to avoid duplicates.

### **How to Set Up the Scheduled Job**:

1. **Job Trigger**: Set the job to run on an hourly basis using a `tokio::time::interval` that triggers the data fetch every 60 minutes.
2. **Fetch New Data**: The `fetch_and_store_data` function is called to fetch fresh data from the Midgard API.
3. **Update MongoDB**: After fetching the data, the MongoDB database is updated with new entries.

Example of a background task setup in `main.rs`:
```rust
let mut interval = interval(Duration::from_secs(60 * 60));  // Every hour

loop {
    interval.tick().await;
    fetch_and_store_data(Arc::clone(&db)).await;  // Fetch and store new data
}
```

---

## 3. **API Endpoints for Querying Data**

After populating the MongoDB database with historical data, the following **API endpoints** are available for querying and interacting with the stored data:

### **Available Endpoints**:

1. **`GET /api/depth-history`**:
   - **Purpose**: Retrieve depth history data from the database.
   - **Query Parameters**:
     - `interval`: Defines the time period (e.g., `hour`, `day`, `week`).
     - `count`: Defines how many records to return.
     - `from`, `to`: Define the time range (Unix timestamps).
     - **Pagination**: `page` and `limit` for controlling result set size and offset.
     - **Sort Order**: `sort_order` for ascending or descending results.
   - **Response**:
     - A list of depth history records, along with metadata (e.g., `startTime`, `endTime`).

   #### Example Request:
   ```bash
   GET /api/depth-history?interval=hour&count=400&from=1606780899&to=1608825600&page=2&limit=100&sort_order=desc
   ```

2. **`GET /api/earnings-history`**:
   - **Purpose**: Retrieve earnings history data.
   - **Query Parameters**: Same as `/api/depth-history`.

3. **`GET /api/swaps-history`**:
   - **Purpose**: Retrieve swaps history data.
   - **Query Parameters**: Same as `/api/depth-history`.

4. **`GET /api/rune-pool-history`**:
   - **Purpose**: Retrieve rune pool history data.
   - **Query Parameters**: Same as `/api/depth-history`.

### **API Query Parameters**:

- **`interval`**: Specifies the time period (e.g., `hour`, `day`, `week`, `month`, `quarter`, `year`). If larger intervals are selected, data is aggregated on the backend.
- **`count`**: Specifies the number of records to return (e.g., 100, 400).
- **`from`, `to`**: Specify the Unix timestamp range for the query. This ensures that the query only returns data from the requested time range.
- **`page`**: Allows pagination of results. For example, `page=2` will fetch results from the second page of results.
- **`limit`**: Limits the number of results per page (e.g., `limit=100`).
- **`sort_by`**: Determines the order of results. It can either be `asc` for ascending or `desc` for descending. This allows sorting data by `startTime` or `endTime`.

### **API Response Structure**:

Each endpoint returns a JSON response with the following structure:
```json
{
    "meta": {
        "startTime": 1606780899,
        "endTime": 1608825600,
    },
    "intervals": [
        {
            "startTime": 1606780899,
            "endTime": 1606784499,
            "data": {
                "depth": 1234.56,
                "lp_units": 7890.12,
                "member_count": 50
            }
        },
        ...
    ]
}
```

---


## 4. **Postman Documentation**

### **Postman Collection**:

- You can import the following Postman collection to interact with the API:
  - **Base URL**: `http://localhost:3000`
  - **Endpoints**: `/api/depth-history`, `/api/earnings-history`, `/api/swaps-history`, `/api/rune-pool-history`

- Postman allows you to test the API endpoints with various query parameters and view the responses.

---

## Conclusion

This project provides a robust backend system for fetching, storing, and querying historical data from the **Midgard API**. The data is stored in MongoDB, and custom API endpoints enable users to interact with the data. The scheduled job ensures that the database is updated every hour with the latest data from the API. Swagger/Postman documentation allows for easy API exploration and testing. Advanced query parameters like pagination, sorting, and count limits allow fine-grained control over API responses.
```
