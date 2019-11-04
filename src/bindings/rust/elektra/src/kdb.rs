use crate::ReadableKey;
use crate::{KeySet, StringKey, WriteableKey};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

/// General methods to access the Key database.
/// For example usage see [the Readme](https://github.com/ElektraInitiative/libelektra/tree/master/src/bindings/rust).
#[derive(Debug)]
pub struct KDB {
    ptr: NonNull<elektra_sys::KDB>,
    _marker: std::marker::PhantomData<elektra_sys::KDB>,
}

impl Drop for KDB {
    fn drop(&mut self) {
        let mut err_key = StringKey::new_empty();
        unsafe {
            elektra_sys::kdbClose(self.as_ptr(), err_key.as_ptr());
        }
    }
}

impl KDB {
    /// Opens the session with the Key database.
    pub fn open<'a>() -> Result<Self, KDBError<'a>> {
        let mut key = StringKey::new_empty();
        let kdb_ptr = unsafe { elektra_sys::kdbOpen(key.as_ptr()) };

        if kdb_ptr.is_null() {
            Err(KDBError::new(key.duplicate()))
        } else {
            Ok(KDB {
                ptr: NonNull::new(kdb_ptr).unwrap(),
                _marker: std::marker::PhantomData,
            })
        }
    }

    /// Retrieve keys in an atomic and universal way.
    /// Note that the provided keyset is modified and contains the result.
    /// The provided `key` is used to give a hint about which keys should be retrieved.
    /// The return value is true, if the keys were successfully retrieved
    /// and false if there were no changes to the keyset.
    pub fn get<'a>(
        &mut self,
        keyset: &mut KeySet,
        key: &mut StringKey<'a>,
    ) -> Result<bool, KDBError<'a>> {
        let ret_val = unsafe { elektra_sys::kdbGet(self.as_ptr(), keyset.as_ptr(), key.as_ptr()) };

        if ret_val == 1 {
            Ok(true)
        } else if ret_val == 0 {
            Ok(false)
        } else {
            Err(KDBError::new(key.duplicate()))
        }
    }

    /// Set keys in an atomic and universal way.
    /// The provided `key` is used to give a hint about which keys should be stored.
    /// The return value is true on success,
    /// and false if there were no changes to the KDB.
    /// # Notes
    /// You have to call [`get`](#method.get) with `keyset` first.
    pub fn set<'a>(
        &mut self,
        keyset: &mut KeySet,
        key: &mut StringKey<'a>,
    ) -> Result<bool, KDBError<'a>> {
        let ret_val = unsafe { elektra_sys::kdbSet(self.as_ptr(), keyset.as_ptr(), key.as_ptr()) };

        if ret_val == 1 {
            Ok(true)
        } else if ret_val == 0 {
            Ok(false)
        } else {
            Err(KDBError::new(key.duplicate()))
        }
    }

    /// This method can be used the given KDB handle meets certain clauses, specified in contract.
    /// The return value is true on success,
    /// and false if clauses of the contract are unmet.
    pub fn ensure<'a>(
        &mut self,
        keyset: &mut KeySet,
        key: &mut StringKey<'a>,
    ) -> Result<bool, KDBError<'a>> {
        let ret_val =
            unsafe { elektra_sys::kdbEnsure(self.as_ptr(), keyset.as_ptr(), key.as_ptr()) };

        if ret_val == 0 {
            Ok(true)
        } else if ret_val == 1 {
            Ok(false)
        } else {
            Err(KDBError::new(key.duplicate()))
        }
    }
    /// Returns the raw pointer of the KDB object.
    /// Should be used with caution. In particular,
    /// the pointer should only be modified with
    /// `elektra_sys::kdb*` functions, but `kdbClose`
    /// should not be called.
    pub fn as_ptr(&mut self) -> *mut elektra_sys::KDB {
        self.ptr.as_ptr()
    }
}

impl AsRef<elektra_sys::KDB> for KDB {
    fn as_ref(&self) -> &elektra_sys::KDB {
        unsafe { self.ptr.as_ref() }
    }
}

const ELEKTRA_ERROR_PERMANENT: &str = "C01";
const ELEKTRA_ERROR_RESOURCE: &str = "C011";
const ELEKTRA_ERROR_OUT_OF_MEMORY: &str = "C01110";
const ELEKTRA_ERROR_INSTALLATION: &str = "C012";
const ELEKTRA_ERROR_LOGICAL: &str = "C013";
const ELEKTRA_ERROR_INTERNAL: &str = "C01310";
const ELEKTRA_ERROR_INTERFACE: &str = "C01320";
const ELEKTRA_ERROR_PLUGIN_MISBEHAVIOR: &str = "C01330";
const ELEKTRA_ERROR_CONFLICTING_STATE: &str = "C02";
const ELEKTRA_ERROR_VALIDATION: &str = "C03";
const ELEKTRA_ERROR_VALIDATION_SYNTACTIC: &str = "C03100";
const ELEKTRA_ERROR_VALIDATION_SEMANTIC: &str = "C03200";

/// Wraps a key that contains error metakeys
#[derive(Debug)]
pub struct KDBError<'a> {
    error_key: StringKey<'a>,
}

impl<'a> KDBError<'a> {
    /// Constructs a new KDBError from a StringKey.
    /// Only pass keys where the metakeys error/* are set.
    pub fn new(error_key: StringKey) -> KDBError {
        KDBError { error_key }
    }

    fn is_error(&self, error_str: &str) -> bool {
        self.number().starts_with(error_str)
    }

    pub fn is_permanent(&self) -> bool {
        self.is_error(ELEKTRA_ERROR_PERMANENT)
    }

    pub fn is_resource(&self) -> bool {
        self.is_error(ELEKTRA_ERROR_RESOURCE)
    }

    pub fn is_out_of_memory(&self) -> bool {
        self.is_error(ELEKTRA_ERROR_OUT_OF_MEMORY)
    }

    pub fn is_installation(&self) -> bool {
        self.is_error(ELEKTRA_ERROR_INSTALLATION)
    }

    pub fn is_logical(&self) -> bool {
        self.is_error(ELEKTRA_ERROR_LOGICAL)
    }

    pub fn is_internal(&self) -> bool {
        self.is_error(ELEKTRA_ERROR_INTERNAL)
    }

    pub fn is_interface(&self) -> bool {
        self.is_error(ELEKTRA_ERROR_INTERFACE)
    }

    pub fn is_plugin_misbehavior(&self) -> bool {
        self.is_error(ELEKTRA_ERROR_PLUGIN_MISBEHAVIOR)
    }

    pub fn is_conflicting_state(&self) -> bool {
        self.is_error(ELEKTRA_ERROR_CONFLICTING_STATE)
    }

    pub fn is_validation(&self) -> bool {
        self.is_error(ELEKTRA_ERROR_VALIDATION)
    }

    pub fn is_syntactic(&self) -> bool {
        self.is_error(ELEKTRA_ERROR_VALIDATION_SYNTACTIC)
    }

    pub fn is_semantic(&self) -> bool {
        self.is_error(ELEKTRA_ERROR_VALIDATION_SEMANTIC)
    }

    /// Returns the error number or an empty string if unavailable.
    pub fn number(&self) -> String {
        self.error_key_or_empty_string("error/number")
    }

    /// Returns the error reason or an empty string if unavailable.
    pub fn reason(&self) -> String {
        self.error_key_or_empty_string("error/reason")
    }

    /// Returns the module where the error occured or an empty string if unavailable.
    pub fn module(&self) -> String {
        self.error_key_or_empty_string("error/module")
    }

    /// Returns a description of the error or an empty string if unavailable.
    pub fn description(&self) -> String {
        self.error_key_or_empty_string("error/description")
    }

    /// Returns the source file from where the error information comes or an empty string if unavailable.
    pub fn file(&self) -> String {
        self.error_key_or_empty_string("error/file")
    }

    /// Returns the exact line of that source file or an empty string if unavailable.
    pub fn line(&self) -> String {
        self.error_key_or_empty_string("error/line")
    }

    fn error_key_or_empty_string(&self, error_key: &str) -> String {
        if let Ok(meta) = self.error_key.meta(error_key) {
            meta.value().to_owned().to_string()
        } else {
            "".into()
        }
    }

    /// Returns a formatted error message
    pub fn to_error_message(&self) -> String {
        format!(
            "Sorry, module {module} issued error {error_number}:\n{description}: {reason}",
            module = self.module(),
            error_number = self.number(),
            description = self.description(),
            reason = self.reason()
        )
    }
}

impl<'a> Display for KDBError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_error_message())
    }
}

impl<'a> Error for KDBError<'a> {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{KeyBuilder, LookupOption};

    const PARENT_KEY: &str = "user/sw/tests/rust/1/";
    const KEY_1_NAME: &str = "user/sw/tests/rust/1/key_name";
    const KEY_2_NAME: &str = "user/sw/tests/rust/1/key_name/2";

    const KEY_1_VALUE: &str = "key_value_1";
    const KEY_2_VALUE: &str = "key_value_2";

    fn create_test_keys<'a>() -> (StringKey<'a>, StringKey<'a>, StringKey<'a>) {
        let parent_key = StringKey::new(PARENT_KEY).unwrap_or_else(|e| panic!("{}", e));
        let key1: StringKey = KeyBuilder::new(KEY_1_NAME)
            .unwrap()
            .value(KEY_1_VALUE)
            .build();
        let key2: StringKey = KeyBuilder::new(KEY_2_NAME)
            .unwrap()
            .value(KEY_2_VALUE)
            .build();
        (parent_key, key1, key2)
    }

    #[test]
    fn can_use_kdb() {
        let (mut parent_key, key1, key2) = create_test_keys();
        {
            let mut kdb = KDB::open().unwrap_or_else(|e| panic!("{}", e));
            let mut ks = KeySet::with_capacity(10);
            kdb.get(&mut ks, &mut parent_key)
                .unwrap_or_else(|e| panic!("{}", e));
            ks.append_key(key1);
            ks.append_key(key2);
            kdb.set(&mut ks, &mut parent_key)
                .unwrap_or_else(|e| panic!("{}", e));
        }
        {
            let mut kdb = KDB::open().unwrap_or_else(|e| panic!("{}", e));
            let mut ks = KeySet::with_capacity(2);
            kdb.get(&mut ks, &mut parent_key)
                .unwrap_or_else(|e| panic!("{}", e));
            let key1_lookup = ks
                .lookup_by_name(KEY_1_NAME, Default::default())
                .unwrap()
                .duplicate();
            assert_eq!(key1_lookup.value(), KEY_1_VALUE);

            let key2_lookup = ks
                .lookup_by_name(KEY_2_NAME, Default::default())
                .unwrap()
                .duplicate();
            assert_eq!(key2_lookup.value(), KEY_2_VALUE);
        }
        remove_test_keys();
    }

    fn remove_test_keys() {
        let (mut parent_key, _, _) = create_test_keys();
        let mut kdb = KDB::open().unwrap_or_else(|e| panic!("{}", e));
        let mut ks = KeySet::with_capacity(10);
        kdb.get(&mut ks, &mut parent_key)
            .unwrap_or_else(|e| panic!("{}", e));
        ks.lookup_by_name(KEY_1_NAME, LookupOption::KDB_O_POP)
            .unwrap();
        ks.lookup_by_name(KEY_2_NAME, LookupOption::KDB_O_POP)
            .unwrap();
        kdb.set(&mut ks, &mut parent_key)
            .unwrap_or_else(|e| panic!("{}", e));
    }
}
