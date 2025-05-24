import requests
import uuid
from utils import check_response

user_issues = {}
url = "http://localhost:2000/api/"

class TestUser:
    def __init__(self):
        self.name = f"Test User {uuid.uuid4()}"
        self.email = f"{uuid.uuid4()}@example.com"
        self.password = str(uuid.uuid4())  # Convert UUID to string
        self.id = None
        self.phone = f"+1{uuid.uuid4()}"
        
    
def delete_user(user: TestUser, cleanup: bool = False):
    response = requests.post(url + "DeleteUser", json={"id": user.id})
    if check_response(response):
        print("user deletion successful")
    else:
        user_issues["user deletion failed"] = response.status_code, response.json()

    print("confirming full deletion")
    print("logging in")
    response = requests.post(url + "Login", json={"email": user.email, "password": user.password})
    if not check_response(response):
        print("login failed, deletion successful")
    else:
        print("login successful, account deletion failed")
        user_issues["account deletion failed"] = response.status_code, response.json()

    print("getting name")
    response = requests.post(url + "GetName", json={"id": user.id})
    if not check_response(response):
        print("user name deletion successful, user does not exist")
    else:
        print("user name deletion failed, user still exists")
        user_issues["user name deletion failed"] = response.status_code, response.json()

    print("getting email")
    response = requests.post(url + "GetEmail", json={"id": user.id})
    if not check_response(response):
        print("user email deletion successful, user does not exist")
    else:
        print("user email deletion failed, user still exists")
        user_issues["user email deletion failed"] = response.status_code, response.json()

    print("getting phone")
    response = requests.post(url + "GetPhone", json={"id": user.id})
    if not check_response(response):
        print("user phone deletion successful, user does not exist")
    else:
        print("user phone deletion failed, user still exists")
        user_issues["user phone deletion failed"] = response.status_code, response.json()


def user_tests(part_of_full_suite: bool = False, recursed: bool = False):
    if recursed:
        print("running in recursed mode for additional user creation")
    if part_of_full_suite:
        print("running in full suite mode")
    print("testing user creation")
    first_user = TestUser()
    response = requests.post(url + "CreateUser", json={"name": first_user.name, "email": first_user.email, "password": first_user.password})
    if check_response(response):
        print("user creation successful")
    else:
        return "user creation failed cannot continue"
    
    print("testing user login")
    response = requests.post(url + "Login", json={"email": first_user.email, "password": first_user.password})
    if check_response(response):
        print("user login successful")
        first_user.id = response.json()["id"]
    else:
        return "user login failed cannot continue"
    
    print("testing user name update")
    response = requests.post(url + "UpdateName", json={"id": first_user.id, "name": first_user.name})
    if check_response(response):
        print("user name update successful")
    else:
        user_issues["user name update failed"] = response.status_code, response.json()
    
    print("testing user email update")
    response = requests.post(url + "UpdateEmail", json={"id": first_user.id, "email": first_user.email})
    if check_response(response):
        print("user email update successful")
    else:
        user_issues["user email update failed"] = response.status_code, response.json()
    
    print("testing user phone update")
    response = requests.post(url + "UpdatePhone", json={"id": first_user.id, "phone": first_user.phone})
    if check_response(response):
        print("user phone update successful")
    else:
        user_issues["user phone update failed"] = response.status_code, response.json()
        
    print("testing user name retrieval")
    response = requests.post(url + "GetName", json={"id": first_user.id})
    if check_response(response):
        print("user name retrieval successful")
    else:
        user_issues["user name retrieval failed"] = response.status_code, response.json()
    
    print("testing user email retrieval")
    response = requests.post(url + "GetEmail", json={"id": first_user.id})
    if check_response(response):
        print("user email retrieval successful")
    else:
        user_issues["user email retrieval failed"] = response.status_code, response.json()
        
    print("testing user phone retrieval")
    response = requests.post(url + "GetPhone", json={"id": first_user.id})
    if check_response(response):
        print("user phone retrieval successful")
    else:
        user_issues["user phone retrieval failed"] = response.status_code, response.json()
    
    print("testing user deletion")
    if recursed:
        return first_user
    if part_of_full_suite:
        print("doing full suite test, creating second user to delete")
        second_user = user_tests(part_of_full_suite=False, recursed=True)
        delete_user(second_user)
        print("creating new second user for friend tests later")
        second_user = user_tests(part_of_full_suite=False, recursed=True)
        return (first_user, second_user, user_issues)
    else:
        delete_user(first_user)
        return f"all issues: {user_issues}"
