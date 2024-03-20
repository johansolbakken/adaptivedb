# Skyline module

- The existing general query execution methods are not fast enough.
- BNL variants good for small skylines of low dimensionality
    - Sensitive to the number of dimensions and correlation
        - BNL is best up to 5 dimensions
            - Lowest I/O cost, even for 10 dimensions
        - Especially replacement is a bad idea if the skyline is large
        - Better choice the larger the data set is
            - Performs well also for small buffer sizes
- D&C performs better with large buffer sizes
    - D&C-mptesk is the best D&C algorithm
        - The I/O cost is too high for the other variants
- “In summary, we propose that a system should implement the D&C-mptesk and BNL-sol algorithms”