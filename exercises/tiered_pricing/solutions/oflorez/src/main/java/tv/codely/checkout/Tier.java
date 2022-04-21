package tv.codely.checkout;


public enum Tier {

    FIRST(1,2,299),
    SECOND(3,10,239),
    THIRD(11,25,219);

    private final int lowerLimit;
    private final int upperLimit;
    private final int unitPrice;

    Tier(int lowerLimit, int upperLimit, int unitPrice) {
        this.lowerLimit = lowerLimit;
        this.upperLimit = upperLimit;
        this.unitPrice = unitPrice;
    }

    public int getLowerLimit() {
        return lowerLimit;
    }

    public int getUpperLimit() {
        return upperLimit;
    }

    public int getUnitPrice() {
        return unitPrice;
    }

    public static int getTotalPrice(int subscriptions) {
        var found = FIRST;
        for (Tier tier : values()) {
            if ( subscriptions >= tier.lowerLimit && subscriptions <= tier.upperLimit) {
                found = tier;
                break;
            }
        }

        return subscriptions * found.unitPrice;
    }
}
