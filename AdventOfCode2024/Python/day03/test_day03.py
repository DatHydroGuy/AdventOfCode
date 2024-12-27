import unittest
import puzzles


class MyTestCase(unittest.TestCase):
    def test_nothing(self):
        # Arrange
        expected = None

        # Act
        result = None

        # Assert
        self.assertEqual(result, expected)


if __name__ == '__main__':
    unittest.main()
