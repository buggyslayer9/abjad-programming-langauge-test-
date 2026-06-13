# أبجد - المُجمِّع (Abjad Compiler)

المُجمِّع الرسمي للغة البرمجة أبجد - لغة برمجة عربية عالية الأداء.

The official compiler for the Abjad programming language - an Arabic-first, high-performance programming language.

## الحالة (Status)

**المرحلة:** ٠ - التأسيس (Foundation)  
**الإصدار:** ٠.١.٠-alpha  
**التقدم:** Lexer أساسي مُنفذ (Basic lexer implemented)

## الميزات المنفذة (Implemented Features)

### المُجمِّع الأساسي (Basic Compiler)
- [x] إعداد مشروع Rust للمُجمِّع
- [x] تنفيذ Token types
- [x] تنفيذ Lexer (محلل معجمي) أساسي
- [ ] تنفيذ Parser (محلل نحوي)
- [ ] بناء AST (شجرة البنية المجردة)
- [ ] تنفيذ Type Checker (مدقق الأنواع)
- [ ] إعداد LLVM backend
- [ ] تنفيذ IR generation
- [ ] إعداد Cranelift backend (بديل)
- [ ] تنفيذ Object file generation
- [ ] إعداد Linker integration

### الأدوات الأساسية (Basic Tools)
- [x] إعداد مشروع CLI للمُجمِّع
- [x] تنفيذ هيكل الأوامر الأساسية
- [ ] تنفيذ `abjad build`
- [ ] تنفيذ `abjad run`
- [ ] تنفيذ `abjad check`
- [ ] تنفيذ `abjad clean`
- [ ] إعداد error messages بالعربية
- [ ] إعداد warning messages
- [ ] تنفيذ basic error recovery

## التثبيت (Installation)

```bash
# Clone the repository
git clone https://github.com/abjad-lang/abjad.git
cd abjad/compiler

# Build the compiler
cargo build --release

# Add to PATH (optional)
export PATH=$PATH:$(pwd)/target/release
```

## الاستخدام (Usage)

```bash
# Build a file
abjad build main.abjad

# Run a file
abjad run main.abjad

# Check code without compiling
abjad check main.abjad

# Clean build artifacts
abjad clean

# Format code
abjad fmt main.abjad

# Initialize a new project
abjad init myproject

# Add a dependency
abjad add package-name

# Show help
abjad --help
```

## مثال (Example)

```abjad
// hello.abjad
دالة رئيسية() {
    طباعة("مرحباً بالعالم!")
}
```

```bash
$ abjad build hello.abjad
Building: hello.abjad
Output: hello
Optimization level: 0
Debug symbols: false
```

## البنية (Structure)

```
compiler/
├── Cargo.toml          # Project configuration
├── README.md           # This file
└── src/
    ├── main.rs         # CLI entry point
    ├── lib.rs          # Library root
    ├── cli.rs          # CLI commands
    ├── error.rs        # Error types
    ├── token.rs        # Token definitions
    └── lexer.rs        # Lexer implementation
```

## التطوير (Development)

```bash
# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- build hello.abjad

# Format code
cargo fmt

# Check code
cargo check

# Run clippy
cargo clippy
```

## المساهمة (Contributing)

نرحب بالمساهمات! يرجى قراءة [CONTRIBUTING.md](../CONTRIBUTING.md) للحصول على التفاصيل.

We welcome contributions! Please read [CONTRIBUTING.md](../CONTRIBUTING.md) for details.

## الترخيص (License)

هذا المشروع مرخص تحت رخصة MIT أو Apache-2.0.

This project is licensed under the MIT or Apache-2.0 license.

## روابط (Links)

- [الموقع الرسمي](https://abjad.dev)
- [التوثيق](https://docs.abjad.dev)
- [خارطة الطريق](../refrences/phases-roadmap.md)
- [وثيقة متطلبات المنتج](../refrences/abjad-prd.html)

---

**أبجد** - لغة برمجة عربية للمستقبل  
**Abjad** - An Arabic programming language for the future
