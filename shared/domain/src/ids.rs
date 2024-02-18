/*      (Clone), (Copy) позволяется легко скопировать IDS
        (Debug) сможет распечатывать нам журналы,
        (Eq) позволяет нам проверить равен ли один идентификатор другому,
        (Hash) позволит нам хешировать идентификатор в хеш таблице,
        (Deserialize) позволяет читать из сети,
        а (Serialize) позволить нам включить их в джйсон и отправит их по сети,
        (PartialEq) это часть (Eq) позволяет нам выполнить сравнение,
        а потом частичный порядок позволит нам сортировать их
*/

#[derive(
    Clone, Copy, Debug, Eq, Hash,
serde::Deserialize, serde:: Serialize,
PartialEq, Ord, PartialOrd,
)]
//прописаны в заисимостях отдельно эту ф-ю для бекенда, чтобы не извлекатьт ID
//мы проверяем функции запроса и если мы ее получим, мы добавляем еще одну производную версию
//которая будет нового типа Diesel. DieselNewType позволяет нам использовать ID пользователя, когда мы запускаем запросы
#[cfg_attr(feature = "query", derive(DieselNewType))]
pub struct UserId(uuid::Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
    
    pub fn into_inner(self) -> uuid::Uuid {
        self.0
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
    //значения по умолчанию создадим в ручную и по умолчанию мы просто создадим пустой ID
    impl Default for UserId {
        fn default() -> Self {
            Self::new()
        }
    }

    //создадим чтобы пользователь мог конвертировать обычный идентификатор в ID пользователя 

    impl From<uuid::Uuid> for UserId {
        fn from(id: uuid::Uuid) -> Self {
            UserId(id)
        }
    }

    impl std::str::FromStr for UserId {
        type Err = IdError;
        fn from_str(id: &str) -> Result<Self, Self::Err> {
            uuid::Uuid:: try_parse(id)
            .map(|id| id.into())
            .map_err(|_| IdError::Parse)
        }
    }
    #[derive(Debug, thiserror::Error)]
    pub enum IdError {
        #[error("failed to parse ID")]
        Parse,
    }