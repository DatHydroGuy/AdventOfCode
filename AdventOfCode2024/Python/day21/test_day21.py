import unittest

from sympy.multipledispatch.dispatcher import source

import puzzles


class NumpadTests(unittest.TestCase):
    def test_init_numpad(self):
        # Arrange
        expected = (3, 2)

        # Act
        numpad = puzzles.NumPad()
        pos = numpad.get_position()

        # Assert
        self.assertEqual(pos, expected)

    def test_init_position(self):
        # Arrange
        expected = 'A'

        # Act
        numpad = puzzles.NumPad()
        key = numpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_left(self):
        # Arrange
        expected = '0'
        numpad = puzzles.NumPad()

        # Act
        numpad.left()
        key = numpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_left_stays_on_pad(self):
        # Arrange
        expected = 'X'
        numpad = puzzles.NumPad()

        # Act
        numpad.left()
        numpad.left()
        numpad.left()
        numpad.left()
        numpad.left()
        numpad.left()
        key = numpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_right(self):
        # Arrange
        expected = '0'
        numpad = puzzles.NumPad()

        # Act
        numpad.left()
        numpad.left()
        numpad.right()
        key = numpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_right_stays_on_pad(self):
        # Arrange
        expected = 'A'
        numpad = puzzles.NumPad()

        # Act
        numpad.right()
        numpad.right()
        key = numpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_up(self):
        # Arrange
        expected = '3'
        numpad = puzzles.NumPad()

        # Act
        numpad.up()
        key = numpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_up_stays_on_pad(self):
        # Arrange
        expected = '9'
        numpad = puzzles.NumPad()

        # Act
        numpad.up()
        numpad.up()
        numpad.up()
        numpad.up()
        numpad.up()
        key = numpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_down(self):
        # Arrange
        expected = '6'
        numpad = puzzles.NumPad()

        # Act
        numpad.up()
        numpad.up()
        numpad.up()
        numpad.down()
        key = numpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_down_stays_on_pad(self):
        # Arrange
        expected = 'A'
        numpad = puzzles.NumPad()

        # Act
        numpad.down()
        numpad.down()
        key = numpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_from_A_to_7(self):
        # Arrange
        expected = '7'
        numpad = puzzles.NumPad()

        # Act
        numpad.move_to_key('7')
        key = numpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_all_move(self):
        # Arrange
        source = ['7', '8', '9', '4', '5', '6', '1', '2', '3', '0', 'A']
        dest = ['7', '8', '9', '4', '5', '6', '1', '2', '3', '0', 'A']
        numpad = puzzles.NumPad()
        all_good = True

        # Act
        for s in source:
            numpad.move_to_key(s)
            for d in dest:
                if s == d:
                    continue

                numpad.move_to_key(d)
                key = numpad.get_curr_key()
                all_good = all_good and (key == d)

        # Assert
        self.assertTrue(all_good)



class DirpadTests(unittest.TestCase):
    def test_init_numpad(self):
        # Arrange
        expected = (0, 2)

        # Act
        dirpad = puzzles.DirPad()
        pos = dirpad.get_position()

        # Assert
        self.assertEqual(pos, expected)

    def test_init_position(self):
        # Arrange
        expected = 'A'

        # Act
        dirpad = puzzles.NumPad()
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_left(self):
        # Arrange
        expected = '^'
        dirpad = puzzles.DirPad()

        # Act
        dirpad.left()
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_left_stays_on_pad(self):
        # Arrange
        expected = 'X'
        dirpad = puzzles.DirPad()

        # Act
        dirpad.left()
        dirpad.left()
        dirpad.left()
        dirpad.left()
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_right(self):
        # Arrange
        expected = '^'
        dirpad = puzzles.DirPad()

        # Act
        dirpad.left()
        dirpad.left()
        dirpad.right()
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_right_stays_on_pad(self):
        # Arrange
        expected = 'A'
        dirpad = puzzles.DirPad()

        # Act
        dirpad.right()
        dirpad.right()
        dirpad.right()
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_up(self):
        # Arrange
        expected = 'A'
        dirpad = puzzles.DirPad()

        # Act
        dirpad.down()
        dirpad.up()
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_up_stays_on_pad(self):
        # Arrange
        expected = 'A'
        dirpad = puzzles.DirPad()

        # Act
        dirpad.up()
        dirpad.up()
        dirpad.up()
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_down(self):
        # Arrange
        expected = '>'
        dirpad = puzzles.DirPad()

        # Act
        dirpad.down()
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_down_stays_on_pad(self):
        # Arrange
        expected = '>'
        dirpad = puzzles.DirPad()

        # Act
        dirpad.down()
        dirpad.down()
        dirpad.down()
        dirpad.down()
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_from_A_to_left(self):
        # Arrange
        expected = '<'
        dirpad = puzzles.DirPad()

        # Act
        dirpad.move_to_key('<')
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_from_A_to_up(self):
        # Arrange
        expected = '^'
        dirpad = puzzles.DirPad()

        # Act
        dirpad.move_to_key('^')
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_from_A_to_down(self):
        # Arrange
        expected = 'v'
        dirpad = puzzles.DirPad()

        # Act
        dirpad.move_to_key('v')
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_move_from_A_to_right(self):
        # Arrange
        expected = '>'
        dirpad = puzzles.DirPad()

        # Act
        dirpad.move_to_key('>')
        key = dirpad.get_curr_key()

        # Assert
        self.assertEqual(key, expected)

    def test_all_move(self):
        # Arrange
        source = ['^', 'A', '<', 'v', '>']
        dest = ['^', 'A', '<', 'v', '>']
        dirpad = puzzles.DirPad()
        all_good = True

        # Act
        for s in source:
            dirpad.move_to_key(s)
            for d in dest:
                if s == d:
                    continue

                dirpad.move_to_key(d)
                key = dirpad.get_curr_key()
                all_good = all_good and (key == d)

        # Assert
        self.assertTrue(all_good)


if __name__ == '__main__':
    unittest.main()
