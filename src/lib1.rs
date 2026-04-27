use std::{borrow::Cow, path::PathBuf};
use icu::locale::{Locale, locale};
use icu_provider::{DataError, DataErrorKind, DataMarker, DataPayload, DataProvider, DataRequest, DataResponse, DataResponseMetadata, data_marker, data_struct};

data_marker!(DocumentV1, &'static str);

// 2. Создаем наш кастомный провайдер, который знает, где лежат файлы.
pub struct DocumentProvider;

// 3. Реализуем логику загрузки данных.
impl DataProvider<DocumentV1> for DocumentProvider {
    fn load(&self, request: DataRequest) -> Result<DataResponse<DocumentV1>, DataError> {
        // Получаем запрошенную локаль в виде строки (например, "ru-RU", "ru" или "")
        let locale = request.id.locale.to_string();

        let data = match &*locale {
            "en" => asset!("/assets/en/fatty_acids/Monounsaturated.md"),
            "ru" => asset!("/assets/ru/fatty_acids/Monounsaturated.md"),
            _ => asset!("/assets/en/fatty_acids/Monounsaturated.md"),
        };
        Ok(DataResponse {
            // Метаданные можно оставить дефолтными, LocaleFallbackProvider сам их обновит
            metadata: DataResponseMetadata::default(),
            payload: DataPayload::from_owned(data),
        })

        // // Если локаль пустая ("und" - undetermined), это корень цепочки фолбэка.
        // // Назначим для него папку по умолчанию, например "en".
        // let folder_name = if locale.is_empty() { "en" } else { &locale };
        // // Формируем путь: base_dir/ru/content.md
        // let file_path = self.base_dir.join(folder_name).join("content.md");
        // // Пытаемся прочитать файл
        // match std::fs::read_to_string(&file_path) {
        //     Ok(content) => {
        //         Ok(DataResponse {
        //             // Метаданные можно оставить дефолтными, LocaleFallbackProvider сам их обновит
        //             metadata: DataResponseMetadata::default(),
        //             payload: DataPayload::from_owned("MarkdownV1 { content: Cow::Owned(content) }"),
        //         })
        //     }
        //     Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
        //         // КРИТИЧЕСКИ ВАЖНО: Возвращаем IdentifierNotFound.
        //         // Увидев эту ошибку, LocaleFallbackProvider попробует следующую локаль.
        //         Err(DataErrorKind::IdentifierNotFound.with_req(MarkdownV1::INFO, request))
        //     }
        //     Err(_) => {
        //         // Любая другая ошибка (нет прав доступа и т.д.)
        //         Err(DataErrorKind::Custom.with_req(MarkdownV1::INFO, request))
        //     }
        // }
    }
}

pub mod fatty_acids;
pub mod math;

mod macros;

#[cfg(test)]
mod test {
    use super::*;
    use icu::locale::LocaleFallbacker;
    use icu_provider::{DataIdentifierBorrowed, DataIdentifierCow};
    use icu_provider_adapters::fallback::LocaleFallbackProvider;
    use std::error::Error;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        let locale = locale!("ru");
        let provider = LocaleFallbackProvider::new(DocumentProvider, LocaleFallbacker::new().static_to_owned());
        let response = DataProvider::<DocumentV1>::load(&provider, DataRequest { id: DataIdentifierBorrowed::for_locale(&locale.into()), ..Default::default() })?;
        Ok(())
    }
}
