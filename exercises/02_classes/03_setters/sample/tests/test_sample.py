# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
from unittest import TestCase

from setters import Item


class ItemTests(TestCase):
    def setUp(t):
        t.item = Item("Cart", 10)

    def test_setter_access_count(t):
        t.assertEqual(t.item.n_visits, 0)

        _ = t.item.name
        t.assertEqual(t.item.n_visits, 1)

        _ = t.item.price
        t.assertEqual(t.item.n_visits, 2)

        _ = t.item.price
        t.assertEqual(t.item.n_visits, 3)
