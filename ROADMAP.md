# İlk Sürüm Roadmap

Bu roadmap yalnızca ilk basit yerel Logic Pro arşiv uygulamasını kapsar. Her
aşama küçük, çalıştırılabilir ve manuel olarak test edilebilir olacak şekilde
planlandı.

## 1. Tauri + React + TypeScript Başlangıç Projesi

- Amaç: Tauri 2, React, TypeScript, Vite, Rust ve npm ile minimal masaüstü
  uygulama temelini kurmak.
- Değişecek dosyalar: `package.json`, `vite.config.ts`, `tsconfig.json`,
  `index.html`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`.
- Tamamlanma kriteri: `npm install` başarılı olur ve standart Tauri proje
  yapısı oluşur.
- Nasıl test edilir: Frontend için `npm run dev`, Rust/Cargo kurulduktan sonra
  masaüstü uygulaması için `npm run tauri dev` çalıştırılır.

## 2. Temel İki Ekran Yapısı

- Amaç: Router eklemeden proje listesi ve proje detay ekranlarının yerini
  hazırlamak.
- Değişecek dosyalar: `src/App.tsx`, `src/pages/ProjectsPage.tsx`,
  `src/pages/ProjectDetailPage.tsx`, `src/styles.css`.
- Tamamlanma kriteri: Uygulama `Logic Version Archive` başlığı, `Add Logic
  Project` butonu ve empty-state mesajı ile açılır.
- Nasıl test edilir: Uygulama çalıştırılır ve ilk ekran görsel olarak kontrol
  edilir.

## 3. `.logicx` Dosya/Package Seçimi

- Amaç: Kullanıcının macOS native dialog ile Logic Pro package seçmesini
  sağlamak.
- Değişecek dosyalar: `src/components/AddProjectButton.tsx`,
  `src/api/tauriCommands.ts`, `src-tauri/src/commands.rs`.
- Tamamlanma kriteri: Butona basınca native picker açılır ve seçilen path
  frontend’e döner.
- Nasıl test edilir: Bir `.logicx` package seçilir ve seçilen yol UI’da geçici
  olarak gösterilir.

## 4. Rust Tarafında Path Doğrulama

- Amaç: Seçilen path kabul edilmeden önce `.logicx` uzantılı bir package klasörü
  olup olmadığını doğrulamak.
- Değişecek dosyalar: `src-tauri/src/file_service.rs`,
  `src-tauri/src/commands.rs`.
- Tamamlanma kriteri: Geçerli `.logicx` package kabul edilir, normal klasör veya
  dosya için anlaşılır hata döner.
- Nasıl test edilir: Bir `.logicx` package, normal klasör ve normal dosya
  seçilerek sonuçlar karşılaştırılır.

## 5. Application Data Klasörünün Oluşturulması

- Amaç: Uygulama verilerini Tauri app data directory altında saklamak.
- Değişecek dosyalar: `src-tauri/src/file_service.rs`,
  `src-tauri/src/project_service.rs`.
- Tamamlanma kriteri: Uygulama hardcoded kullanıcı yolu kullanmadan `projects/`
  klasörünü oluşturabilir.
- Nasıl test edilir: İlgili command çalıştırılır ve macOS app data altında
  klasörün oluştuğu doğrulanır.

## 6. Proje Modeli Ve JSON Metadata

- Amaç: Proje ve versiyon metadata’sını `project.json` içinde saklamak.
- Değişecek dosyalar: `src-tauri/src/models.rs`,
  `src-tauri/src/project_service.rs`, `src/types/project.ts`.
- Tamamlanma kriteri: Rust tarafı bir projeyi versiyonlarıyla birlikte JSON’a
  yazıp geri okuyabilir.
- Nasıl test edilir: Geçici command veya unit test ile örnek `project.json`
  yazılıp okunur.

## 7. İlk Versiyonun Recursive Kopyalanması

- Amaç: Seçilen `.logicx` package projesini `versions/v001/` altına tam kopya
  olarak almak.
- Değişecek dosyalar: `src-tauri/src/file_service.rs`,
  `src-tauri/src/project_service.rs`, `src-tauri/src/version_service.rs`.
- Tamamlanma kriteri: Proje ekleme işlemi Version 1 klasörünü recursive olarak
  oluşturur.
- Nasıl test edilir: Küçük bir test `.logicx` package klasörü eklenir ve iç
  dosyaların kopyalandığı kontrol edilir.

## 8. Proje Listesinin Gösterilmesi

- Amaç: JSON metadata’dan arşivlenmiş projeleri listelemek.
- Değişecek dosyalar: `src/pages/ProjectsPage.tsx`,
  `src/components/ProjectCard.tsx`, `src/api/tauriCommands.ts`,
  `src-tauri/src/commands.rs`.
- Tamamlanma kriteri: Uygulama yeniden açıldığında daha önce eklenen projeler
  listede görünür.
- Nasıl test edilir: Bir proje eklenir, uygulama yeniden başlatılır ve listenin
  korunduğu doğrulanır.

## 9. Yeni Versiyon Oluşturma

- Amaç: Kaynak Logic projesinin yeni tam kopyasını bir sonraki versiyon numarası
  ile saklamak.
- Değişecek dosyalar: `src/components/SaveVersionForm.tsx`,
  `src-tauri/src/version_service.rs`, `src-tauri/src/project_service.rs`,
  `src-tauri/src/commands.rs`.
- Tamamlanma kriteri: `v002`, `v003` gibi klasörler kısa notlarla birlikte
  oluşturulabilir.
- Nasıl test edilir: Test proje klasörü değiştirilir, yeni versiyon kaydedilir
  ve klasör/metadata kontrol edilir.

## 10. Versiyon Listesinin Gösterilmesi

- Amaç: Proje detayında versiyonları numara, tarih ve not ile göstermek.
- Değişecek dosyalar: `src/pages/ProjectDetailPage.tsx`,
  `src/components/VersionCard.tsx`, `src/types/project.ts`.
- Tamamlanma kriteri: Tüm versiyonlar tutarlı sırayla listelenir.
- Nasıl test edilir: Birden fazla versiyon kaydedilir ve UI’da doğru
  göründükleri kontrol edilir.

## 11. Working Copy Oluşturma

- Amaç: Saklanan bir versiyonu kullanıcının seçtiği yeni konuma kopyalamak.
- Değişecek dosyalar: `src/components/VersionCard.tsx`,
  `src/api/tauriCommands.ts`, `src-tauri/src/version_service.rs`,
  `src-tauri/src/commands.rs`.
- Tamamlanma kriteri: Seçilen versiyon ayrı bir working copy olarak dışa
  aktarılabilir.
- Nasıl test edilir: Version 1 geçici bir klasöre dışa aktarılır ve Finder’da
  açılır.

## 12. Finder’da Gösterme

- Amaç: Saklanan versiyonun konumunu Finder’da açmak.
- Değişecek dosyalar: `src/components/VersionCard.tsx`,
  `src/api/tauriCommands.ts`, `src-tauri/src/commands.rs`,
  `src-tauri/capabilities/default.json`.
- Tamamlanma kriteri: Versiyon aksiyonu doğru klasörü Finder’da gösterir.
- Nasıl test edilir: Mevcut bir versiyonda reveal aksiyonu çalıştırılır ve
  Finder’ın doğru konumda açıldığı kontrol edilir.

## 13. Hata Yönetimi Ve Manuel Testler

- Amaç: Ürün büyümeden önce sık karşılaşılacak hataları anlaşılır hale getirmek.
- Değişecek dosyalar: `src/pages/ProjectsPage.tsx`,
  `src/pages/ProjectDetailPage.tsx`, `src-tauri/src/commands.rs`, servis
  modülleri, `README.md`.
- Tamamlanma kriteri: Yaygın hatalar UI’da anlaşılır mesajlarla görünür ve
  manuel test adımları dokümante edilir.
- Nasıl test edilir: Geçersiz path, silinmiş kaynak proje, çakışan export hedefi
  ve permission-denied senaryoları denenir.
