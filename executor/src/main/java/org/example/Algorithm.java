package org.example;

public enum Algorithm {
    EUCLIDEAN("euclidean"),
    MANHATTAN("manhattan"),
    CHEBYSHEV("chebyshev");

    private final String value;

    Algorithm(String value) {
        this.value = value;
    }

    public String getValue() {
        return value;
    }
}
