# DAW Version Control

DAW Version Control, Logic Pro `.logicx` package projelerini yerel olarak tam
kopyalar halinde versiyonlamak için hazırlanan macOS odaklı bir masaüstü
uygulamasıdır. İlk milestone bilinçli olarak küçük tutuldu: uygulama iskeleti,
ilk ekran ve ileride dosya işlemlerini taşıyacak sade Rust modülleri hazırlandı.

## Şu Anki Kapsam

- macOS odaklı yerel masaüstü uygulaması.
- Yalnızca Logic Pro `.logicx` package projeleri.
- Her versiyonu tam proje kopyası olarak saklama yaklaşımı.
- React ile ilk proje listesi ekranı.
- Rust/Tauri tarafında gelecekteki filesystem işlemleri için başlangıç yapısı.
- Sonraki milestone’larda metadata JSON dosyaları Tauri app data klasörü altında
  saklanacak.

## Kapsam Dışı

- Logic proje içeriğini parse etme.
- Track, channel, clip, plugin veya parametre diff analizi.
- Git entegrasyonu, branch, cloud sync, kullanıcı hesabı, authentication veya
  collaboration.
- SQLite, PostgreSQL, Docker, compression, deduplication, hash, otomatik dosya
  izleme ve Ableton desteği.

## Kullanılan Teknolojiler

- Tauri 2
- React
- TypeScript
- Vite
- Rust
- Serde
- serde_json
- Tauri Dialog Plugin
- Tauri Opener Plugin
- npm

## Geliştirme Ortamı Gereksinimleri

Bu projeyi çalıştırmak için şunlar gerekir:

- Node.js
- npm
- Rust
- Cargo

Kurulum sırasında Node.js ve npm mevcuttu. Bu shell içinde Rust ve Cargo
bulunamadı; bu yüzden `npm run tauri dev` komutunu çalıştırmadan önce Rust
kurulmalıdır.

## Uygulamayı Çalıştırma Komutları

Bağımlılıkları kur:

```bash
npm install
```

Sadece Vite frontend’i çalıştır:

```bash
npm run dev
```

Rust/Cargo kurulduktan sonra Tauri masaüstü uygulamasını çalıştır:

```bash
npm run tauri dev
```

Frontend build kontrolü:

```bash
npm run build
```

## Ana Klasör Yapısı

```text
src/
├── api/
├── components/
├── pages/
├── types/
├── App.tsx
├── main.tsx
└── styles.css

src-tauri/src/
├── commands.rs
├── file_service.rs
├── lib.rs
├── main.rs
├── models.rs
├── project_service.rs
└── version_service.rs
```

## İlk Sürümün Kullanıcı Akışı

Hedeflenen ilk gerçek ürün akışı:

1. Kullanıcı bir `.logicx` Logic Pro package projesi seçer.
2. Uygulama seçilen yolun geçerli bir Logic package olduğunu doğrular.
3. Package, uygulama saklama alanına Version 1 olarak kopyalanır.
4. Proje, proje listesinde görünür.
5. Kullanıcı daha sonra Version 2, Version 3 gibi yeni tam kopyalar kaydeder.
6. Versiyonlar numara, tarih ve not ile listelenir.
7. Kullanıcı saklanan bir versiyonu Finder’da gösterebilir.
8. Kullanıcı eski bir versiyonu yeni bir working copy olarak dışa aktarabilir.

Bu milestone gerçek proje seçme veya kopyalama yapmadan, yalnızca uygulama
iskeletini ve ilk ekranı tamamlar.
