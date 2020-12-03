class PGQueryParser {
    static {
        System.loadLibrary("libpg_query_rust");
    }

    private static native String parse(String input);

    public static void main(String[] args) {
        String output = PGQueryParser.parse("SELECT 1");
        System.out.println(output);
    }
}