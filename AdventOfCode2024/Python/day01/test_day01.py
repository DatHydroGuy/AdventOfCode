import unittest
import puzzles


class MyTestCase(unittest.TestCase):
    def test_dist_between_numbers(self):
        # Arrange
        expected = 6

        # Act
        result = puzzles.dist_between_numbers(3, 9)

        # Assert
        self.assertEqual(result, expected)


if __name__ == '__main__':
    unittest.main()
