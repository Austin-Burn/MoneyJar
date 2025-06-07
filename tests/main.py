from user_tests import user_tests
from friends_tests import friend_tests
from event_tests import event_tests

def main():
    print("Available test suites:")
    print("1. Users")
    print("2. Friends")
    print("3. Events")
    print("4. WhoInWhat")
    print("5. All")

    choice = input("\nEnter the number of the test suite to run: ")

    if choice == "1":
        user_tests()
    elif choice == "2":
        friend_tests()
    elif choice == "3":
        event_tests()
    elif choice == "4":
        WhoInWhatTests().run_all()
    elif choice == "5":
        results = {}
        user_results = user_tests(part_of_full_suite=True)
        results["user_results"] = user_results[2]
        friend_results = friend_tests((user_results[0], user_results[1]))
        results["friend_results"] = friend_results
        event_results = event_tests((user_results[0], user_results[1]))
        results["event_results"] = event_results
        print(results)
    else:
        print("Invalid choice!")

if __name__ == "__main__":
    main() 