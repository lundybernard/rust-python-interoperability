# Modify the Python package under `src` to satisfy the tests.
# Do NOT modify the tests themselves!
import pytest

from mprocessing import word_count

def test_word_count_single_process():
    text = "hello world"
    assert word_count(text, 1) == 2


def test_word_count_multiple_processes():
    text = "hello world"
    assert word_count(text, 2) == 2


def test_word_count_multiple_processes_long_text():
    text = "hello world " * 1000
    assert word_count(text, 2) == 2000


def test_more_processes_than_words():
    text = "hello world"
    assert word_count(text, 10) == 2
