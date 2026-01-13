# DePIN Orcha - API Reference Guide

**Version:** 1.0.0  
**Date:** January 13, 2026  
**Base URL:** `http://localhost:8080`

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Metrics Endpoints](#metrics-endpoints)
3. [Optimization Endpoints](#optimization-endpoints)
4. [Reallocation Endpoints](#reallocation-endpoints)
5. [Dashboard Endpoints](#dashboard-endpoints)
6. [Alert Endpoints](#alert-endpoints)
7. [System Endpoints](#system-endpoints)
8. [WebSocket](#websocket)
9. [Error Handling](#error-handling)
10. [Examples](#examples)

---

## Getting Started

### Base Configuration

```bash
# Default Configuration
Host: 127.0.0.1
Port: 8080
Version: v1
Content-Type: application/json
```

### Response Format

All responses follow this format:

**Success:**

```json
{
  "success": true,
  "data": {
    /* response data */
  },
  "timestamp": "2026-01-13T12:00:00Z"
}
```

**Error:**

```json
{
  "error": "ERROR_CODE",
  "message": "Description of error",
  "timestamp": "2026-01-13T12:00:00Z"
}
```

---

## Metrics Endpoints

### 1. Get Current Metrics

**Request:**

```http
GET /api/v1/metrics
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "timestamp": "2026-01-13T12:00:00Z",
    "total_earnings_per_hour": 45.75,
    "earnings_by_protocol": {
      "streamr": 12.5,
      "storj": 15.25,
      "golem": 10.0,
      "grass": 8.0
    },
    "allocation_by_protocol": {
      "streamr": 25.0,
      "storj": 35.0,
      "golem": 25.0,
      "grass": 15.0
    },
    "connection_status": {
      "streamr": true,
      "storj": true,
      "golem": false,
      "grass": true
    },
    "resource_utilization": {
      "cpu_percent": 45.2,
      "memory_percent": 62.5,
      "bandwidth_percent": 38.1,
      "storage_percent": 71.3
    }
  },
  "timestamp": "2026-01-13T12:00:00Z"
}
```

### 2. Get Metrics History

**Request:**

```http
GET /api/v1/metrics/history?hours=24&limit=100
```

**Parameters:**

- `hours` (optional): Number of hours to query (default: 24)
- `limit` (optional): Maximum records to return (default: 1000)

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "metrics": [
      {
        "timestamp": "2026-01-13T12:00:00Z",
        "total_earnings": 45.75,
        "earnings_by_protocol": {
          "streamr": 12.5,
          "storj": 15.25,
          "golem": 10.0,
          "grass": 8.0
        }
      },
      {
        /* more records */
      }
    ],
    "total_count": 24
  },
  "timestamp": "2026-01-13T12:00:00Z"
}
```

---

## Optimization Endpoints

### 3. Get Optimization Opportunities

**Request:**

```http
GET /api/v1/opportunities?limit=10
```

**Parameters:**

- `limit` (optional): Maximum opportunities to return (default: 10)

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "opportunities": [
      {
        "from_protocol": "grass",
        "to_protocol": "storj",
        "current_rate": 8.0,
        "projected_rate": 18.5,
        "earnings_improvement": 10.5,
        "confidence": 0.92
      },
      {
        "from_protocol": "golem",
        "to_protocol": "storj",
        "current_rate": 0.0,
        "projected_rate": 15.25,
        "earnings_improvement": 15.25,
        "confidence": 0.88
      }
    ],
    "best_improvement": 15.25,
    "confidence": 0.92
  },
  "timestamp": "2026-01-13T12:00:00Z"
}
```

### 4. Get Optimal Allocation

**Request:**

```http
GET /api/v1/allocation
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "current_allocation": {
      "streamr": 25.0,
      "storj": 35.0,
      "golem": 25.0,
      "grass": 15.0
    },
    "optimal_allocation": {
      "streamr": 20.0,
      "storj": 55.0,
      "golem": 0.0,
      "grass": 25.0
    },
    "estimated_improvement": 18.75,
    "net_benefit": 2.45,
    "roi_percent": 5.37
  },
  "timestamp": "2026-01-13T12:00:00Z"
}
```

---

## Reallocation Endpoints

### 5. Execute Reallocation

**Request:**

```http
POST /api/v1/reallocate
Content-Type: application/json

{
  "allocation": {
    "streamr": 20.0,
    "storj": 55.0,
    "golem": 0.0,
    "grass": 25.0
  },
  "reason": "Optimization: improved ROI by 5.37%"
}
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "success": true,
    "message": "Reallocation executed successfully",
    "changes": [
      {
        "timestamp": "2026-01-13T12:05:00Z",
        "protocol": "grass",
        "old_allocation": 15.0,
        "new_allocation": 25.0,
        "earnings_impact": 1.25
      },
      {
        "timestamp": "2026-01-13T12:05:00Z",
        "protocol": "storj",
        "old_allocation": 35.0,
        "new_allocation": 55.0,
        "earnings_impact": 8.5
      }
    ]
  },
  "timestamp": "2026-01-13T12:00:00Z"
}
```

**Error Responses:**

```http
400 Bad Request
{
  "error": "INVALID_ALLOCATION",
  "message": "Total allocation 98.5 != 100%",
  "timestamp": "2026-01-13T12:00:00Z"
}

429 Too Many Requests
{
  "error": "CANNOT_REALLOCATE",
  "message": "Reallocation not currently allowed (rate limit or hold duration)",
  "timestamp": "2026-01-13T12:00:00Z"
}
```

### 6. Get Reallocation History

**Request:**

```http
GET /api/v1/reallocation/history
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": [
    {
      "timestamp": "2026-01-13T12:05:00Z",
      "protocol": "grass",
      "old_allocation": 15.0,
      "new_allocation": 25.0,
      "earnings_impact": 1.25
    },
    {
      "timestamp": "2026-01-13T11:00:00Z",
      "protocol": "streamr",
      "old_allocation": 30.0,
      "new_allocation": 20.0,
      "earnings_impact": -1.5
    }
  ],
  "timestamp": "2026-01-13T12:00:00Z"
}
```

---

## Dashboard Endpoints

### 7. Get Dashboard

**Request:**

```http
GET /api/v1/dashboard
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "timestamp": "2026-01-13T12:00:00Z",
    "total_earnings_per_hour": 45.75,
    "earnings_by_protocol": {
      "streamr": 12.5,
      "storj": 15.25,
      "golem": 10.0,
      "grass": 8.0
    },
    "current_allocation": {
      "streamr": 25.0,
      "storj": 35.0,
      "golem": 25.0,
      "grass": 15.0
    },
    "optimal_allocation": {
      "streamr": 20.0,
      "storj": 55.0,
      "golem": 0.0,
      "grass": 25.0
    },
    "next_reallocation_in": 3600,
    "connection_status": {
      "streamr": true,
      "storj": true,
      "golem": false,
      "grass": true
    },
    "alerts_count": 1
  },
  "timestamp": "2026-01-13T12:00:00Z"
}
```

---

## Alert Endpoints

### 8. Get Alerts

**Request:**

```http
GET /api/v1/alerts
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "alerts": [
      {
        "timestamp": "2026-01-13T10:30:00Z",
        "alert_type": "LOW_EARNINGS",
        "severity": 0.7,
        "message": "Golem earnings below threshold",
        "acknowledged": false
      },
      {
        "timestamp": "2026-01-13T09:15:00Z",
        "alert_type": "CONNECTION_LOST",
        "severity": 0.85,
        "message": "Golem connection lost for 15 minutes",
        "acknowledged": true
      }
    ],
    "total_count": 2,
    "critical_count": 0
  },
  "timestamp": "2026-01-13T12:00:00Z"
}
```

### 9. Acknowledge Alert

**Request:**

```http
POST /api/v1/alerts/acknowledge
Content-Type: application/json

{
  "timestamp": "2026-01-13T10:30:00Z"
}
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "acknowledged": true
  },
  "timestamp": "2026-01-13T12:00:00Z"
}
```

---

## System Endpoints

### 10. Health Check

**Request:**

```http
GET /api/v1/health
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "status": "healthy"
  },
  "timestamp": "2026-01-13T12:00:00Z"
}
```

### 11. Get Status

**Request:**

```http
GET /api/v1/status
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "protocols": ["streamr", "storj", "golem", "grass"],
    "timestamp": "2026-01-13T12:00:00Z"
  },
  "timestamp": "2026-01-13T12:00:00Z"
}
```

---

## WebSocket

### 12. Real-Time Updates

**Connection:**

```javascript
const ws = new WebSocket("ws://localhost:8080/ws");

ws.onopen = () => {
  // Subscribe to metrics
  ws.send(
    JSON.stringify({
      type: "Subscribe",
      protocol: null, // null = all protocols
    })
  );
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log("Update:", message);
};
```

**Message Types:**

1. **Subscribe**

```json
{
  "type": "Subscribe",
  "protocol": "storj" // or null for all
}
```

2. **Unsubscribe**

```json
{
  "type": "Unsubscribe",
  "protocol": "storj" // or null for all
}
```

3. **MetricsUpdate** (incoming)

```json
{
  "type": "MetricsUpdate",
  "metrics": {
    "timestamp": "2026-01-13T12:00:00Z",
    "total_earnings": 45.75,
    "earnings_by_protocol": {
      /* ... */
    }
  }
}
```

4. **AlertNotification** (incoming)

```json
{
  "type": "AlertNotification",
  "alert": {
    "timestamp": "2026-01-13T12:00:00Z",
    "alert_type": "LOW_EARNINGS",
    "severity": 0.7,
    "message": "...",
    "acknowledged": false
  }
}
```

5. **Ping/Pong**

```json
{ "type": "Ping" }
{ "type": "Pong" }
```

---

## Error Handling

### Common Error Codes

| Code               | HTTP Status | Description                          |
| ------------------ | ----------- | ------------------------------------ |
| NO_DATA            | 404         | No metrics available yet             |
| INTERNAL_ERROR     | 500         | Server error                         |
| INVALID_ALLOCATION | 400         | Invalid allocation percentages       |
| CANNOT_REALLOCATE  | 429         | Rate limited or hold duration active |
| NOT_FOUND          | 404         | Resource not found                   |
| ANALYSIS_ERROR     | 500         | Failed to analyze data               |

---

## Examples

### Python Example

```python
import requests
import json

BASE_URL = 'http://localhost:8080'

# Get current metrics
response = requests.get(f'{BASE_URL}/api/v1/metrics')
metrics = response.json()['data']
print(f"Current earnings: ${metrics['total_earnings_per_hour']:.2f}/hr")

# Get opportunities
opportunities = requests.get(f'{BASE_URL}/api/v1/opportunities?limit=5').json()
for opp in opportunities['data']['opportunities']:
    print(f"{opp['from_protocol']} -> {opp['to_protocol']}: "
          f"+${opp['earnings_improvement']:.2f}")

# Get dashboard
dashboard = requests.get(f'{BASE_URL}/api/v1/dashboard').json()['data']
print(f"Total alerts: {dashboard['alerts_count']}")
```

### JavaScript Example

```javascript
async function getDashboard() {
  const response = await fetch("http://localhost:8080/api/v1/dashboard");
  const { data } = await response.json();

  console.log(`Earnings: $${data.total_earnings_per_hour.toFixed(2)}/hr`);
  console.log("Allocation:", data.current_allocation);
  console.log("Alerts:", data.alerts_count);

  return data;
}

// WebSocket real-time updates
const ws = new WebSocket("ws://localhost:8080/ws");
ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  if (message.type === "MetricsUpdate") {
    updateDashboard(message.metrics);
  }
};
```

### cURL Examples

```bash
# Get metrics
curl http://localhost:8080/api/v1/metrics

# Get opportunities
curl "http://localhost:8080/api/v1/opportunities?limit=5"

# Get dashboard
curl http://localhost:8080/api/v1/dashboard

# Get alerts
curl http://localhost:8080/api/v1/alerts

# Acknowledge alert
curl -X POST http://localhost:8080/api/v1/alerts/acknowledge \
  -H "Content-Type: application/json" \
  -d '{"timestamp": "2026-01-13T10:30:00Z"}'

# Execute reallocation
curl -X POST http://localhost:8080/api/v1/reallocate \
  -H "Content-Type: application/json" \
  -d '{
    "allocation": {
      "streamr": 20.0,
      "storj": 55.0,
      "golem": 0.0,
      "grass": 25.0
    },
    "reason": "Optimization"
  }'
```

---

## Rate Limiting & Quotas

- **Metrics**: 60 requests/minute
- **Reallocation**: 1 per hour
- **Dashboard**: 30 requests/minute
- **WebSocket**: Unlimited (per connection)

---

**Last Updated:** January 13, 2026  
**API Version:** 1.0.0  
**Maintainer:** DePIN Orcha Team
