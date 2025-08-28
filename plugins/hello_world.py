"""
Hello World Plugin for Rest Reminder
This plugin is for showcasing how to build a plugin
"""

# Plugin will be ignored by indicating this constant to be 1
_SHOULD_IGNORE = 1

def on_init(context):
    """
    Will be called when Rest Reminder is initiating
    Context is a dict that stores 3 pieces of information:
        message,
        timestamp,
        work_duration
    """
    print("Hello World! Rest Reminder has been initialized!")
    print(f"Message: {context['message']}")
    print(f"Timestamp: {context['timestamp']}")
    print("Hello World plugin is ready!")

def on_work_start(context):
    """
    Will be called when starting to work
    """
    print("Hello! You're starting to work!")
    print(f"Message: {context['message']}")
    print(f"Time: {context['timestamp']}")
    print("Good luck with your work session!")

def on_break_reminder(context):
    """
    Will be called when break is needed
    """
    print("[Python Plugin] Hello! Time for a break!")
    print(f"[Python Plugin] Message: {context['message']}")
    print(f"[Python Plugin] Work duration: {context['work_duration']} seconds")
    print("[Python Plugin] Remember to stretch and hydrate!")

    # More logic can be built here
    duration_minutes = context['work_duration'] / 60
    if duration_minutes > 60:
        print("[Python Plugin] Wow! You've been working for over an hour! Great job!")
    elif duration_minutes > 30:
        print("[Python Plugin] Good work session! Time to recharge!")
    else:
        print("[Python Plugin] Short but productive session! Keep it up!")

def custom_function():
    """
    Custom function that won't be called by Rest Reminder but can be called by other hooks
    """
    return "This is a custom function from Hello World plugin!"

# Plugin info (optional)
PLUGIN_INFO = {
    "name": "Hello World Plugin",
    "version": "0.0.1",
    "description": "A simple test plugin that says hello",
    "author": "Rest Reminder Team"
}

# print("Hello World plugin loaded!")
