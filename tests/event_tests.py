from user_tests import TestUser
import requests
from utils import check_response

url = "http://localhost:2000/api/"
event_issues = {}

def event_tests(part_of_full_suite=False):
    print("running event tests")
    print("testing event creation")
    response = requests.post(url + "CreateEvent", json={"id": user1, "name": "test event", "description": "test description", "date": "2021-01-01"})
    if check_response(response):
        print("event creation successful")
    else:
        print("event creation failed")
        event_issues["event creation failed"] = response.status_code, response.json()
    return event_issues
