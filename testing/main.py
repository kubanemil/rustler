def number_to_words(n):
    if n < 0 or n > 1000:
        raise ValueError("Number out of range (1-1000)")

    ones = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
            "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"]
    tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"]
    thousand = "onethousand"
    hundred = "hundred"
    and_ = "and"

    if n == 0:
        return "zero"
    if n == 1000:
        return thousand

    # Separate the number into hundreds, tens, and ones
    h = n // 100
    t = (n % 100) // 10
    o = n % 10

    # Build the word representation
    word = ""
    if h > 0:
        word += ones[h] + " " + hundred
    if h > 0 and (t > 0 or o > 0):
        word += " " + and_
    if t > 0:
        word += tens[t]
    if t > 0 and o > 0:
        word += " " + ones[o]
    elif o > 0:
        word += ones[o]

    s = "".join(word.split(" "))
    print(len(s))
    return s.strip()  # Remove any potential leading/trailing spaces

# Example usage
number = 10
word = number_to_words(number)
print(word)  # Output: three hundred and twenty one
