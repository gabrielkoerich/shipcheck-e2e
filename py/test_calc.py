def add(a, b):
    return a + b


def test_add():
    assert add(2, 2) == 4


def test_add_wrong_on_purpose():
    assert add(2, 2) == 5


def test_add_strings():
    assert add("a", "b") == "ab"
