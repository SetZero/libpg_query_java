class PGQueryParser {
    // This declares that the static `hello` method will be provided
    // a native library.
    private static native String parse(String input);

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("libpg_query_rust");
    }

    public static void main(String[] args) {
        String output = PGQueryParser.parse("max");
        System.out.println(output);
    }
}