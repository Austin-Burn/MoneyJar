from user_tests import TestUser
import requests
from utils import check_response

url = "http://localhost:2000/api/"
event_issues = {}

def event_tests(real_users=None):
    if real_users is not None:
        user1 = real_users[0]
        user2 = real_users[1]
    else:
        user1 = TestUser()
        response = requests.post(url + "CreateUser", json={"name": user1.name, "email": user1.email, "password": user1.password})
        response = requests.post(url + "Login", json={"email": user1.email, "password": user1.password})
        user1.id = response.json()["id"]
        print(f"user1 id: {user1.id}")
        user2 = TestUser()
        response = requests.post(url + "CreateUser", json={"name": user2.name, "email": user2.email, "password": user2.password})
        response = requests.post(url + "Login", json={"email": user2.email, "password": user2.password})
        user2.id = response.json()["id"]
        print(f"user2 id: {user2.id}")
    print("running event tests")
    print("testing event creation")
    response = requests.post(url + "CreateEvent", json={"owner_id": user1.id, "name": "test event", "reoccuring": False})
    print(f"CreateEvent response: {response.json()}")
    if check_response(response):
        print("event creation successful")
    else:
        print("event creation failed")
        event_issues["event creation failed"] = response.status_code, response.json()
        return event_issues

    print("getting event id")
    print(f"user1.id being used for GetAllEvents: {user1.id}")
    response = requests.post(url + "GetAllEvents", json={"id": user1.id})
    print(f"get all events response: {response.json()}")
    event_id = response.json()["events"][0]["id"]

    print("testing event update owner id")
    response = requests.post(url + "UpdateEventOwnerId", json={"id": event_id, "owner_id": user2.id})
    if check_response(response):
        print("event update owner id successful")
    else:
        print("event update owner id failed")
        event_issues["event update owner id failed"] = response.status_code, response.json()
    
    print("testing event update name")
    response = requests.post(url + "UpdateEventName", json={"id": event_id, "name": "test event 2"})
    if check_response(response):
        print("event update name successful")
    else:
        print("event update name failed")
        event_issues["event update name failed"] = response.status_code, response.json()
    
    print("testing event update description")
    response = requests.post(url + "UpdateEventDescription", json={"id": event_id, "description": "test description 2"})
    if check_response(response):
        print("event update description successful")
    else:
        print("event update description failed")
        event_issues["event update description failed"] = response.status_code, response.json()
    
    print("testing event update date")
    response = requests.post(url + "UpdateEventDate", json={"id": event_id, "date": "2021-01-02"})
    if check_response(response):
        print("event update date successful")
    else:
        print("event update date failed")
        event_issues["event update date failed"] = response.status_code, response.json()

    print("testing event update reoccuring")
    response = requests.post(url + "UpdateEventReoccuring", json={"id": event_id, "reoccuring": True})
    if check_response(response):
        print("event update reoccuring successful")
    else:
        print("event update reoccuring failed")
        event_issues["event update reoccuring failed"] = response.status_code, response.json()

    print("testing event update reoccuring interval")
    response = requests.post(url + "UpdateEventReoccuringInterval", json={"id": event_id, "reoccuring_interval": "daily"})
    if check_response(response):
        print("event update reoccuring interval successful")
    else:
        print("event update reoccuring interval failed")
        event_issues["event update reoccuring interval failed"] = response.status_code, response.json()

    print("testing event update final date")
    response = requests.post(url + "UpdateEventFinalDate", json={"id": event_id, "final_date": "2021-01-03"})
    if check_response(response):
        print("event update final date successful")
    else:
        print("event update final date failed")
        event_issues["event update final date failed"] = response.status_code, response.json()

    print("testing event deletion")
    response = requests.post(url + "DeleteEvent", json={"id": event_id})
    if check_response(response):
        print("event deletion successful")
    else:
        print("event deletion failed")
        event_issues["event deletion failed"] = response.status_code, response.json()

    print("testing event deletion with invalid id")
    response = requests.post(url + "DeleteEvent", json={"id": "invalid_id"})
    if check_response(response):
        print("event deletion with invalid id successful")
    else:
        print("event deletion with invalid id failed")
        event_issues["event deletion with invalid id failed"] = response.status_code, response.json()

    print("testing change name with after deletion")
    response = requests.post(url + "UpdateEventName", json={"id": event_id, "name": "test event 3"})
    if check_response(response):
        print("change name with after deletion successful")
    else:
        print("change name with after deletion failed")
        event_issues["change name with after deletion failed"] = response.status_code, response.json()

    print("testing change description with after deletion")
    response = requests.post(url + "UpdateEventDescription", json={"id": event_id, "description": "test description 3"})
    if check_response(response):
        print("change description with after deletion successful")
    else:
        print("change description with after deletion failed")
        event_issues["change description with after deletion failed"] = response.status_code, response.json()

    print("testing change date with after deletion")
    response = requests.post(url + "UpdateEventDate", json={"id": event_id, "date": "2021-01-04"})
    if check_response(response):
        print("change date with after deletion successful")
    else:
        print("change date with after deletion failed")
        event_issues["change date with after deletion failed"] = response.status_code, response.json()

    print("testing change reoccuring with after deletion")
    response = requests.post(url + "UpdateEventReoccuring", json={"id": event_id, "reoccuring": False})
    if check_response(response):
        print("change reoccuring with after deletion successful")
    else:
        print("change reoccuring with after deletion failed")
        event_issues["change reoccuring with after deletion failed"] = response.status_code, response.json()

    print("testing change reoccuring interval with after deletion")
    response = requests.post(url + "UpdateEventReoccuringInterval", json={"id": event_id, "reoccuring_interval": "weekly"})
    if check_response(response):
        print("change reoccuring interval with after deletion successful")
    else:
        print("change reoccuring interval with after deletion failed")
        event_issues["change reoccuring interval with after deletion failed"] = response.status_code, response.json()

    print("testing change final date with after deletion")
    response = requests.post(url + "UpdateEventFinalDate", json={"id": event_id, "final_date": "2021-01-05"})
    if check_response(response):
        print("change final date with after deletion successful")
    else:
        print("change final date with after deletion failed")
        event_issues["change final date with after deletion failed"] = response.status_code, response.json()

    print("testing change final date with after deletion")
    response = requests.post(url + "UpdateEventFinalDate", json={"id": event_id, "final_date": "2021-01-05"})
    if check_response(response):    
        print("change final date with after deletion successful")
    else:
        print("change final date with after deletion failed")
        event_issues["change final date with after deletion failed"] = response.status_code, response.json()
    return (event_issues, True)
