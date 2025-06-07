import requests
from typing import Dict, Any, Tuple
import json



def check_response(response):
    response_data = response.json() if response.content else "None"
    if response.status_code != 200:
        return False
    else:
        return True