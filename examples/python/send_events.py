import json
import os
import time
import uuid

import requests

API_BASE = os.getenv("API_BASE", "http://localhost:8080")
API_KEY = os.getenv("API_KEY", "")

if not API_KEY:
    raise SystemExit("Please set API_KEY before running this script.")

for i in range(5):
    payload = {
        "event_name": "page_view",
        "page_url": f"/article/{i}",
        "session_id": f"session-{uuid.uuid4()}",
        "properties": {
            "language": "python",
            "source": "example-client",
            "iteration": i,
        },
    }

    response = requests.post(
        f"{API_BASE}/api/v1/events",
        headers={
            "Content-Type": "application/json",
            "X-API-Key": API_KEY,
        },
        data=json.dumps(payload),
        timeout=5,
    )

    print(f"[{i}] status={response.status_code} body={response.text}")
    time.sleep(1)
