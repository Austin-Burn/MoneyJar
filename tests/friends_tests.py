import requests
from utils import check_response
from user_tests import TestUser
import uuid

url = "http://localhost:2000/api/"

friend_issues = {}

def friend_tests(real_users: tuple[TestUser, TestUser] = None):
    print("testing friend creation")
    if real_users is not None:
        print("continueing with full suite")
        response = requests.post(url + "CreateFriend", json={"id": real_users[0].id, "friend_id": real_users[1].id})
        if check_response(response):
            print("friend creation successful")
        else:
            print("friend creation failed")
            friend_issues["friend creation failed"] = response.status_code, response.json()
        
        print("testing friend deletion")
        response = requests.post(url + "DeleteFriend", json={"id": real_users[0].id, "friend_id": real_users[1].id})
        if check_response(response):
            print("friend deletion successful")
        else:
            print("friend deletion failed")
            friend_issues["friend deletion failed"] = response.status_code, response.json()
    else:
        user1 = str(uuid.uuid4())
        user2 = str(uuid.uuid4())
        response = requests.post(url + "CreateFriend", json={"id": user1, "friend_id": user2})
        if check_response(response):
            print("friend creation successful")
        else:
            print("friend creation failed")
            friend_issues["friend creation failed"] = response.status_code, response.json()
        
        print("testing friend deletion")
        response = requests.post(url + "DeleteFriend", json={"id": user1, "friend_id": user2})
        if check_response(response):
            print("friend deletion successful")
        else:
            print("friend deletion failed")
            friend_issues["friend deletion failed"] = response.status_code, response.json()

    return friend_issues


