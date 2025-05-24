import requests
import json
from typing import Dict, Any
import sys

BASE_URL = "http://localhost:2000/api"

def make_request(endpoint: str, method: str, data: Dict[str, Any] = None) -> Dict[str, Any]:
    """Helper function to make HTTP requests"""
    url = f"{BASE_URL}/{endpoint}"
    try:
        if method.upper() == "GET":
            response = requests.get(url, json=data)
        else:  # POST
            response = requests.post(url, json=data)
        return response.json() if response.content else {}
    except Exception as e:
        print(f"Error making request to {endpoint}: {str(e)}")
        return {}

class UserTests:
    def __init__(self):
        self.user_id = None
        self.email = "test@example.com"
        self.password = "testpass123"
        self.name = "Test User"
        self.phone = "1234567890"

    def run_all(self):
        print("\n=== Running User Tests ===")
        self.test_create_user()
        self.test_login()
        self.test_get_name()
        self.test_get_email()
        self.test_get_phone()
        self.test_update_name()
        self.test_update_email()
        self.test_update_phone()
        self.test_delete_user()

    def test_create_user(self):
        print("\nTesting CreateUser...")
        data = {
            "name": self.name,
            "email": self.email,
            "password": self.password
        }
        response = make_request("CreateUser", "POST", data)
        print(f"CreateUser Response: {response}")

    def test_login(self):
        print("\nTesting Login...")
        data = {
            "email": self.email,
            "password": self.password
        }
        response = make_request("Login", "POST", data)
        if response and "id" in response:
            self.user_id = response["id"]
        print(f"Login Response: {response}")

    def test_get_name(self):
        print("\nTesting GetName...")
        data = {"id": self.user_id}
        response = make_request("GetName", "POST", data)
        print(f"GetName Response: {response}")

    def test_get_email(self):
        print("\nTesting GetEmail...")
        data = {"id": self.user_id}
        response = make_request("GetEmail", "POST", data)
        print(f"GetEmail Response: {response}")

    def test_get_phone(self):
        print("\nTesting GetPhone...")
        data = {"id": self.user_id}
        response = make_request("GetPhone", "POST", data)
        print(f"GetPhone Response: {response}")

    def test_update_name(self):
        print("\nTesting UpdateName...")
        data = {
            "id": self.user_id,
            "name": "Updated Name"
        }
        response = make_request("UpdateName", "POST", data)
        print(f"UpdateName Response: {response}")

    def test_update_email(self):
        print("\nTesting UpdateEmail...")
        data = {
            "id": self.user_id,
            "email": "updated@example.com"
        }
        response = make_request("UpdateEmail", "POST", data)
        print(f"UpdateEmail Response: {response}")

    def test_update_phone(self):
        print("\nTesting UpdatePhone...")
        data = {
            "id": self.user_id,
            "phone": "9876543210"
        }
        response = make_request("UpdatePhone", "POST", data)
        print(f"UpdatePhone Response: {response}")

    def test_delete_user(self):
        print("\nTesting DeleteUser...")
        data = {"id": self.user_id}
        response = make_request("DeleteUser", "POST", data)
        print(f"DeleteUser Response: {response}")

class FriendTests:
    def __init__(self):
        self.user_id = None
        self.friend_id = None
        self.user_tests = UserTests()

    def run_all(self):
        print("\n=== Running Friend Tests ===")
        # First create two users
        self.user_tests.test_create_user()
        self.user_tests.test_login()
        self.user_id = self.user_tests.user_id

        # Create second user
        self.user_tests.email = "friend@example.com"
        self.user_tests.test_create_user()
        self.user_tests.test_login()
        self.friend_id = self.user_tests.user_id

        # Now run friend tests
        self.test_create_friend()
        self.test_get_friends()
        self.test_delete_friend()

        # Clean up
        self.user_tests.test_delete_user()
        self.user_tests.user_id = self.friend_id
        self.user_tests.test_delete_user()

    def test_create_friend(self):
        print("\nTesting CreateFriend...")
        data = {
            "id": self.user_id,
            "friend_id": self.friend_id
        }
        response = make_request("CreateFriend", "POST", data)
        print(f"CreateFriend Response: {response}")

    def test_get_friends(self):
        print("\nTesting GetFriends...")
        data = {"id": self.user_id}
        response = make_request("GetFriends", "POST", data)
        print(f"GetFriends Response: {response}")

    def test_delete_friend(self):
        print("\nTesting DeleteFriend...")
        data = {
            "id": self.user_id,
            "friend_id": self.friend_id
        }
        response = make_request("DeleteFriend", "POST", data)
        print(f"DeleteFriend Response: {response}")

class EventTests:
    def __init__(self):
        self.user_id = None
        self.event_id = None
        self.user_tests = UserTests()

    def run_all(self):
        print("\n=== Running Event Tests ===")
        # First create a user
        self.user_tests.test_create_user()
        self.user_tests.test_login()
        self.user_id = self.user_tests.user_id

        # Run event tests
        self.test_create_event()
        self.test_get_event()
        self.test_get_all_events()
        self.test_update_owner_id()
        self.test_update_name()
        self.test_update_description()
        self.test_update_date()
        self.test_update_reoccuring()
        self.test_update_reoccuring_interval()
        self.test_update_final_date()
        self.test_delete_event()

        # Clean up
        self.user_tests.test_delete_user()

    def test_create_event(self):
        print("\nTesting CreateEvent...")
        data = {
            "owner_id": self.user_id,
            "name": "Test Event",
            "reoccuring": True
        }
        response = make_request("CreateEvent", "POST", data)
        if response and "id" in response:
            self.event_id = response["id"]
        print(f"CreateEvent Response: {response}")

    def test_get_event(self):
        print("\nTesting GetEvent...")
        data = {"id": self.event_id}
        response = make_request("GetEvent", "POST", data)
        print(f"GetEvent Response: {response}")

    def test_get_all_events(self):
        print("\nTesting GetAllEvents...")
        data = {"id": self.user_id}
        response = make_request("GetAllEvents", "POST", data)
        print(f"GetAllEvents Response: {response}")

    def test_update_owner_id(self):
        print("\nTesting UpdateOwnerId...")
        data = {
            "id": self.event_id,
            "owner_id": self.user_id
        }
        response = make_request("UpdateOwnerId", "POST", data)
        print(f"UpdateOwnerId Response: {response}")

    def test_update_name(self):
        print("\nTesting UpdateEventName...")
        data = {
            "id": self.event_id,
            "name": "Updated Event Name"
        }
        response = make_request("UpdateEventName", "POST", data)
        print(f"UpdateEventName Response: {response}")

    def test_update_description(self):
        print("\nTesting UpdateDescription...")
        data = {
            "id": self.event_id,
            "description": "Updated event description"
        }
        response = make_request("UpdateDescription", "POST", data)
        print(f"UpdateDescription Response: {response}")

    def test_update_date(self):
        print("\nTesting UpdateDate...")
        data = {
            "id": self.event_id,
            "date": "2024-12-31"
        }
        response = make_request("UpdateDate", "POST", data)
        print(f"UpdateDate Response: {response}")

    def test_update_reoccuring(self):
        print("\nTesting UpdateReoccuring...")
        data = {
            "id": self.event_id,
            "reoccuring": False
        }
        response = make_request("UpdateReoccuring", "POST", data)
        print(f"UpdateReoccuring Response: {response}")

    def test_update_reoccuring_interval(self):
        print("\nTesting UpdateReoccuringInterval...")
        data = {
            "id": self.event_id,
            "reoccuring_interval": "monthly"
        }
        response = make_request("UpdateReoccuringInterval", "POST", data)
        print(f"UpdateReoccuringInterval Response: {response}")

    def test_update_final_date(self):
        print("\nTesting UpdateFinalDate...")
        data = {
            "id": self.event_id,
            "final_date": "2025-12-31"
        }
        response = make_request("UpdateFinalDate", "POST", data)
        print(f"UpdateFinalDate Response: {response}")

    def test_delete_event(self):
        print("\nTesting DeleteEvent...")
        data = {"id": self.event_id}
        response = make_request("DeleteEvent", "POST", data)
        print(f"DeleteEvent Response: {response}")

class WhoInWhatTests:
    def __init__(self):
        self.user_id = None
        self.event_id = None
        self.user_tests = UserTests()
        self.event_tests = EventTests()

    def run_all(self):
        print("\n=== Running WhoInWhat Tests ===")
        # First create a user and event
        self.user_tests.test_create_user()
        self.user_tests.test_login()
        self.user_id = self.user_tests.user_id

        self.event_tests.user_id = self.user_id
        self.event_tests.test_create_event()
        self.event_id = self.event_tests.event_id

        # Run who in what tests
        self.test_create_who_in_what()
        self.test_get_users_from_event()
        self.test_get_events_from_user()
        self.test_delete_who_in_what()

        # Clean up
        self.event_tests.test_delete_event()
        self.user_tests.test_delete_user()

    def test_create_who_in_what(self):
        print("\nTesting CreateWhoInWhat...")
        data = {
            "user_id": self.user_id,
            "event_id": self.event_id
        }
        response = make_request("CreateWhoInWhat", "POST", data)
        print(f"CreateWhoInWhat Response: {response}")

    def test_get_users_from_event(self):
        print("\nTesting GetUsersFromEvent...")
        data = {"event_id": self.event_id}
        response = make_request("GetUsersFromEvent", "POST", data)
        print(f"GetUsersFromEvent Response: {response}")

    def test_get_events_from_user(self):
        print("\nTesting GetEventsFromUser...")
        data = {"user_id": self.user_id}
        response = make_request("GetEventsFromUser", "POST", data)
        print(f"GetEventsFromUser Response: {response}")

    def test_delete_who_in_what(self):
        print("\nTesting DeleteWhoInWhat...")
        data = {
            "user_id": self.user_id,
            "event_id": self.event_id
        }
        response = make_request("DeleteWhoInWhat", "POST", data)
        print(f"DeleteWhoInWhat Response: {response}")

def main():
    print("Available test suites:")
    print("1. Users")
    print("2. Friends")
    print("3. Events")
    print("4. WhoInWhat")
    print("5. All")

    choice = input("\nEnter the number of the test suite to run: ")

    if choice == "1":
        UserTests().run_all()
    elif choice == "2":
        FriendTests().run_all()
    elif choice == "3":
        EventTests().run_all()
    elif choice == "4":
        WhoInWhatTests().run_all()
    elif choice == "5":
        UserTests().run_all()
        FriendTests().run_all()
        EventTests().run_all()
        WhoInWhatTests().run_all()
    else:
        print("Invalid choice!")

if __name__ == "__main__":
    main() 