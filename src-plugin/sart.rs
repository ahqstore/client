#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
pub use models::*;
mod structs {
    #![allow(unused)]
    use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};
    use ahqstore_types::{get_all_commits, Commits};
    use serde::{de::DeserializeOwned, Deserialize, Serialize};
    use tauri::{
        async_runtime::{self, spawn, RwLock},
        plugin::PluginApi, AppHandle, Runtime,
    };
    use tokio::sync::Mutex;
    use crate::{
        models::*,
        structs::{
            daemon::{initialize, IPCSend},
            search::search_daemon,
        },
    };
    pub(crate) mod daemon {
        use std::{
            sync::{Arc, LazyLock, OnceLock},
            thread,
        };
        use ahqstore_types::{Commits, StatusUpdateData};
        use serde::{Deserialize, Serialize};
        use tauri::{AppHandle, Runtime};
        use tokio::{
            runtime::Builder,
            sync::{
                broadcast::{channel, Receiver, Sender as S2},
                mpsc::{unbounded_channel, UnboundedSender as S1},
                RwLock,
            },
        };
        pub enum SendRequest {
            CheckForUpdate,
            PerformTransaction { transaction: u64 },
            PerformAllTransactions,
            CancelTransaction { transaction: u64 },
            InstallUSERAPP { app_id: String },
            RemoveUSERAPP { app_id: String },
        }
        pub type IPCSend = S1<SendRequest>;
        pub type Broadcast = S2<Arc<StatusUpdateData>>;
        pub static BOXED_TX_REF: OnceLock<Broadcast> = OnceLock::new();
        mod desktop {
            use std::{mem::replace, sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};
            use ahqstore_types::{
                AppActionIntent, AppUpdateInstallStatus, Commits, QueuedApp,
                QueuedAppData, StatusUpdateData,
            };
            use tauri::Runtime;
            use tokio::{
                spawn,
                sync::{broadcast::Sender, mpsc::UnboundedReceiver, Notify, RwLock},
                task::JoinHandle, time::{interval, sleep, MissedTickBehavior},
            };
            use crate::structs::{
                daemon::SendRequest, platform, search::CommitSearchIndex, Ahqstore,
            };
            const TEN_MINS: u64 = 10 * 60 * 1000;
            pub mod lock {
                use tokio::sync::Mutex;
                pub static INSTALLLOCK: Mutex<()> = Mutex::const_new(());
                pub fn is_installing() -> bool {
                    INSTALLLOCK.try_lock().is_err()
                }
            }
            pub async fn daemon<R: Runtime>(
                ahqstore: &Ahqstore<R>,
                commits: Arc<RwLock<CommitSearchIndex>>,
                tx: Sender<Arc<StatusUpdateData>>,
                mut rx: UnboundedReceiver<SendRequest>,
            ) {
                let mut user_initiated = false;
                let mut queue: Vec<QueuedApp> = Vec::with_capacity(50);
                let mut intl = interval(Duration::from_mins(10));
                intl.tick().await;
                intl.set_missed_tick_behavior(MissedTickBehavior::Burst);
                let mut notify = Notify::new();
                let mut changed = false;
                let mut transaction = 0;
                loop {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time is running in reverse")
                        .as_secs();
                    {
                        #[doc(hidden)]
                        mod __tokio_select_util {
                            pub(super) enum Out<_0, _1, _2, _3> {
                                _0(_0),
                                _1(_1),
                                _2(_2),
                                _3(_3),
                                Disabled,
                            }
                            pub(super) type Mask = u8;
                        }
                        use ::tokio::macros::support::Future;
                        use ::tokio::macros::support::Pin;
                        use ::tokio::macros::support::Poll::{Ready, Pending};
                        const BRANCHES: u32 = 4;
                        let mut disabled: __tokio_select_util::Mask = Default::default();
                        if !true {
                            let mask: __tokio_select_util::Mask = 1 << 0;
                            disabled |= mask;
                        }
                        if !true {
                            let mask: __tokio_select_util::Mask = 1 << 1;
                            disabled |= mask;
                        }
                        if !true {
                            let mask: __tokio_select_util::Mask = 1 << 2;
                            disabled |= mask;
                        }
                        if !true {
                            let mask: __tokio_select_util::Mask = 1 << 3;
                            disabled |= mask;
                        }
                        let mut output = {
                            let futures_init = (
                                sleep(Duration::from_millis(100)),
                                intl.tick(),
                                notify.notified(),
                                rx.recv(),
                            );
                            let mut futures = (
                                ::tokio::macros::support::IntoFuture::into_future(
                                    futures_init.0,
                                ),
                                ::tokio::macros::support::IntoFuture::into_future(
                                    futures_init.1,
                                ),
                                ::tokio::macros::support::IntoFuture::into_future(
                                    futures_init.2,
                                ),
                                ::tokio::macros::support::IntoFuture::into_future(
                                    futures_init.3,
                                ),
                            );
                            let mut futures = &mut futures;
                            ::tokio::macros::support::poll_fn(|cx| {
                                    match ::tokio::macros::support::poll_budget_available(cx) {
                                        ::core::task::Poll::Ready(t) => t,
                                        ::core::task::Poll::Pending => {
                                            return ::core::task::Poll::Pending;
                                        }
                                    };
                                    let mut is_pending = false;
                                    let start = {
                                        ::tokio::macros::support::thread_rng_n(BRANCHES)
                                    };
                                    for i in 0..BRANCHES {
                                        let branch;
                                        #[allow(clippy::modulo_one)]
                                        {
                                            branch = (start + i) % BRANCHES;
                                        }
                                        match branch {
                                            #[allow(unreachable_code)]
                                            0 => {
                                                let mask = 1 << branch;
                                                if disabled & mask == mask {
                                                    continue;
                                                }
                                                let (fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                let out = match Future::poll(fut, cx) {
                                                    Ready(out) => out,
                                                    Pending => {
                                                        is_pending = true;
                                                        continue;
                                                    }
                                                };
                                                disabled |= mask;
                                                #[allow(unused_variables)] #[allow(unused_mut)]
                                                match &out {
                                                    _ => {}
                                                    _ => continue,
                                                }
                                                return Ready(__tokio_select_util::Out::_0(out));
                                            }
                                            #[allow(unreachable_code)]
                                            1 => {
                                                let mask = 1 << branch;
                                                if disabled & mask == mask {
                                                    continue;
                                                }
                                                let (_, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                let out = match Future::poll(fut, cx) {
                                                    Ready(out) => out,
                                                    Pending => {
                                                        is_pending = true;
                                                        continue;
                                                    }
                                                };
                                                disabled |= mask;
                                                #[allow(unused_variables)] #[allow(unused_mut)]
                                                match &out {
                                                    _ => {}
                                                    _ => continue,
                                                }
                                                return Ready(__tokio_select_util::Out::_1(out));
                                            }
                                            #[allow(unreachable_code)]
                                            2 => {
                                                let mask = 1 << branch;
                                                if disabled & mask == mask {
                                                    continue;
                                                }
                                                let (_, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                let out = match Future::poll(fut, cx) {
                                                    Ready(out) => out,
                                                    Pending => {
                                                        is_pending = true;
                                                        continue;
                                                    }
                                                };
                                                disabled |= mask;
                                                #[allow(unused_variables)] #[allow(unused_mut)]
                                                match &out {
                                                    _ => {}
                                                    _ => continue,
                                                }
                                                return Ready(__tokio_select_util::Out::_2(out));
                                            }
                                            #[allow(unreachable_code)]
                                            3 => {
                                                let mask = 1 << branch;
                                                if disabled & mask == mask {
                                                    continue;
                                                }
                                                let (_, _, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                let out = match Future::poll(fut, cx) {
                                                    Ready(out) => out,
                                                    Pending => {
                                                        is_pending = true;
                                                        continue;
                                                    }
                                                };
                                                disabled |= mask;
                                                #[allow(unused_variables)] #[allow(unused_mut)]
                                                match &out {
                                                    Some(msg) => {}
                                                    _ => continue,
                                                }
                                                return Ready(__tokio_select_util::Out::_3(out));
                                            }
                                            _ => {
                                                ::core::panicking::panic_fmt(
                                                    format_args!(
                                                        "internal error: entered unreachable code: {0}",
                                                        format_args!(
                                                            "reaching this means there probably is an off by one bug",
                                                        ),
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                    if is_pending {
                                        Pending
                                    } else {
                                        Ready(__tokio_select_util::Out::Disabled)
                                    }
                                })
                                .await
                        };
                        match output {
                            __tokio_select_util::Out::_0(_) => {
                                let old = queue.len();
                                queue
                                    .retain(|x| match &x.status {
                                        AppUpdateInstallStatus::Successful { time } => {
                                            now < (*time + 2)
                                        }
                                        AppUpdateInstallStatus::Cancelled { time } => {
                                            now < (*time + 5)
                                        }
                                        AppUpdateInstallStatus::Error { time, .. } => {
                                            now < (*time + 10)
                                        }
                                        _ => true,
                                    });
                                changed = old != queue.len();
                            }
                            __tokio_select_util::Out::_1(_) => {
                                notify.notify_one();
                            }
                            __tokio_select_util::Out::_2(_) => {
                                changed = true;
                            }
                            __tokio_select_util::Out::_3(Some(msg)) => {
                                handle_msg(
                                        ahqstore,
                                        &mut user_initiated,
                                        &notify,
                                        msg,
                                        &mut queue,
                                        &mut transaction,
                                        now,
                                    )
                                    .await;
                                while let Ok(extra_msg) = rx.try_recv() {
                                    handle_msg(
                                            ahqstore,
                                            &mut user_initiated,
                                            &notify,
                                            extra_msg,
                                            &mut queue,
                                            &mut transaction,
                                            now,
                                        )
                                        .await;
                                }
                                changed = true;
                            }
                            __tokio_select_util::Out::Disabled => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "all branches are disabled and there is no else branch",
                                    ),
                                );
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("failed to match bind"),
                                    ),
                                );
                            }
                        }
                    }
                    if changed {
                        let queue_data = queue
                            .iter()
                            .map(QueuedAppData::from)
                            .collect::<Box<[_]>>();
                        _ = tx
                            .send(
                                Arc::new(StatusUpdateData {
                                    disable_update: false,
                                    overflow: queue.len() >= 100,
                                    queue: queue_data,
                                }),
                            );
                        changed = false;
                    }
                }
            }
            #[inline(always)]
            async fn handle_msg<R: Runtime>(
                ahqstore: &Ahqstore<R>,
                user_initiated: &mut bool,
                notify: &Notify,
                msg: SendRequest,
                queue: &mut Vec<QueuedApp>,
                transaction: &mut u64,
                now: u64,
            ) {
                *transaction += 1;
                if queue.len() < 100 {
                    match msg {
                        SendRequest::CheckForUpdate => {
                            if ahqstore.can_update_commit().await {
                                *user_initiated = true;
                                notify.notify_one();
                            }
                        }
                        SendRequest::CancelTransaction { transaction } => {
                            if let Some(x) = queue
                                .iter_mut()
                                .find(|x| x.transaction == transaction)
                            {
                                match x.status {
                                    AppUpdateInstallStatus::Pending
                                    | AppUpdateInstallStatus::PendingUserAction
                                    | AppUpdateInstallStatus::Downloading { .. }
                                    | AppUpdateInstallStatus::AVScanning => {
                                        x.status = AppUpdateInstallStatus::Cancelled {
                                            time: now,
                                        };
                                        if let Some(data) = x.task.take() {
                                            data.abort();
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        SendRequest::PerformTransaction { transaction } => {
                            if let Some(x) = queue
                                .iter_mut()
                                .find(|x| x.transaction == transaction)
                            {
                                x.status = AppUpdateInstallStatus::Pending;
                                x.task = Some(spawn(async {}));
                            }
                        }
                        SendRequest::PerformAllTransactions => {
                            queue
                                .iter_mut()
                                .for_each(|x| {
                                    if let AppUpdateInstallStatus::PendingUserAction = &x.status
                                    {
                                        x.status = AppUpdateInstallStatus::Pending;
                                        x.task = Some(spawn(async {}));
                                    }
                                });
                        }
                        SendRequest::InstallUSERAPP { app_id } => {
                            let app_id: Arc<str> = Arc::from(app_id);
                            queue
                                .push(QueuedApp {
                                    status: AppUpdateInstallStatus::Pending,
                                    intent: AppActionIntent::Install,
                                    transaction: *transaction,
                                    id: app_id,
                                    task: Some(spawn(async {})),
                                });
                        }
                        SendRequest::RemoveUSERAPP { app_id } => {
                            let app_id: Arc<str> = Arc::from(app_id);
                            queue
                                .push(QueuedApp {
                                    status: AppUpdateInstallStatus::Pending,
                                    intent: AppActionIntent::Uninstall,
                                    transaction: *transaction,
                                    id: app_id,
                                    task: Some(spawn(async {})),
                                });
                        }
                    }
                }
            }
        }
        pub use desktop::*;
        use crate::{structs::search::CommitSearchIndex, AhqstoreExt};
        pub fn get_daemon_listener() -> Receiver<Arc<StatusUpdateData>> {
            BOXED_TX_REF.get().expect("Cannot error out").subscribe()
        }
        pub fn initialize<R: Runtime>(
            hwnd: AppHandle<R>,
            commits: Arc<RwLock<CommitSearchIndex>>,
        ) -> IPCSend {
            let (ipc_send, rx) = unbounded_channel();
            let (tx, _) = channel(100);
            BOXED_TX_REF.set(tx.clone()).expect("No error, don't worry");
            thread::spawn(move || {
                Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Unable to create thread")
                    .block_on(async move {
                        let astore = hwnd.ahqstore();
                        daemon(astore, commits, tx, rx).await;
                    });
            });
            ipc_send
        }
    }
    pub(crate) mod platform {
        use std::{fs, thread};
        use ahqstore_types::internet;
        use anyhow::{Context, Error as AnyError};
        use tauri::{command, ipc::Channel, AppHandle};
        use tokio::sync::oneshot;
        pub mod common {
            use std::{fs, path::PathBuf};
            use tauri::{AppHandle, Manager, Runtime};
            use anyhow::Result;
            pub mod desktop {
                use futures::future::join_all;
                use serde::{Deserialize, Serialize};
                use tokio::fs;
                use crate::structs::platform::os::install::{
                    AHQSTORE_USER_DIR, AHQSTORE_GLOBAL_DIR,
                };
                pub async fn list_user_apps() -> Option<Vec<AppListing>> {
                    let dir = &AHQSTORE_USER_DIR.as_str();
                    _inner_list_apps(dir, true).await
                }
                pub async fn list_global_apps() -> Option<Vec<AppListing>> {
                    let dir = &AHQSTORE_GLOBAL_DIR.as_str();
                    _inner_list_apps(dir, false).await
                }
                #[serde(tag = "event", content = "data")]
                pub enum AppListing {
                    Full { app_id: String },
                    Unknown { app_id: String },
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for AppListing {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        match self {
                            AppListing::Full { app_id: __self_0 } => {
                                ::core::fmt::Formatter::debug_struct_field1_finish(
                                    f,
                                    "Full",
                                    "app_id",
                                    &__self_0,
                                )
                            }
                            AppListing::Unknown { app_id: __self_0 } => {
                                ::core::fmt::Formatter::debug_struct_field1_finish(
                                    f,
                                    "Unknown",
                                    "app_id",
                                    &__self_0,
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(
                    non_upper_case_globals,
                    unused_attributes,
                    unused_qualifications,
                    clippy::absolute_paths,
                )]
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl _serde::Serialize for AppListing {
                        fn serialize<__S>(
                            &self,
                            __serializer: __S,
                        ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            match *self {
                                AppListing::Full { ref app_id } => {
                                    #[doc(hidden)]
                                    struct __AdjacentlyTagged<'__a> {
                                        data: (&'__a String,),
                                        phantom: _serde::__private228::PhantomData<AppListing>,
                                    }
                                    #[automatically_derived]
                                    impl<'__a> _serde::Serialize for __AdjacentlyTagged<'__a> {
                                        fn serialize<__S>(
                                            &self,
                                            __serializer: __S,
                                        ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                                        where
                                            __S: _serde::Serializer,
                                        {
                                            #[allow(unused_variables)]
                                            let (app_id,) = self.data;
                                            let mut __serde_state = _serde::Serializer::serialize_struct(
                                                __serializer,
                                                "Full",
                                                0 + 1,
                                            )?;
                                            _serde::ser::SerializeStruct::serialize_field(
                                                &mut __serde_state,
                                                "app_id",
                                                app_id,
                                            )?;
                                            _serde::ser::SerializeStruct::end(__serde_state)
                                        }
                                    }
                                    let mut __struct = _serde::Serializer::serialize_struct(
                                        __serializer,
                                        "AppListing",
                                        2,
                                    )?;
                                    _serde::ser::SerializeStruct::serialize_field(
                                        &mut __struct,
                                        "event",
                                        &_serde::__private228::ser::AdjacentlyTaggedEnumVariant {
                                            enum_name: "AppListing",
                                            variant_index: 0u32,
                                            variant_name: "Full",
                                        },
                                    )?;
                                    _serde::ser::SerializeStruct::serialize_field(
                                        &mut __struct,
                                        "data",
                                        &__AdjacentlyTagged {
                                            data: (app_id,),
                                            phantom: _serde::__private228::PhantomData::<AppListing>,
                                        },
                                    )?;
                                    _serde::ser::SerializeStruct::end(__struct)
                                }
                                AppListing::Unknown { ref app_id } => {
                                    #[doc(hidden)]
                                    struct __AdjacentlyTagged<'__a> {
                                        data: (&'__a String,),
                                        phantom: _serde::__private228::PhantomData<AppListing>,
                                    }
                                    #[automatically_derived]
                                    impl<'__a> _serde::Serialize for __AdjacentlyTagged<'__a> {
                                        fn serialize<__S>(
                                            &self,
                                            __serializer: __S,
                                        ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                                        where
                                            __S: _serde::Serializer,
                                        {
                                            #[allow(unused_variables)]
                                            let (app_id,) = self.data;
                                            let mut __serde_state = _serde::Serializer::serialize_struct(
                                                __serializer,
                                                "Unknown",
                                                0 + 1,
                                            )?;
                                            _serde::ser::SerializeStruct::serialize_field(
                                                &mut __serde_state,
                                                "app_id",
                                                app_id,
                                            )?;
                                            _serde::ser::SerializeStruct::end(__serde_state)
                                        }
                                    }
                                    let mut __struct = _serde::Serializer::serialize_struct(
                                        __serializer,
                                        "AppListing",
                                        2,
                                    )?;
                                    _serde::ser::SerializeStruct::serialize_field(
                                        &mut __struct,
                                        "event",
                                        &_serde::__private228::ser::AdjacentlyTaggedEnumVariant {
                                            enum_name: "AppListing",
                                            variant_index: 1u32,
                                            variant_name: "Unknown",
                                        },
                                    )?;
                                    _serde::ser::SerializeStruct::serialize_field(
                                        &mut __struct,
                                        "data",
                                        &__AdjacentlyTagged {
                                            data: (app_id,),
                                            phantom: _serde::__private228::PhantomData::<AppListing>,
                                        },
                                    )?;
                                    _serde::ser::SerializeStruct::end(__struct)
                                }
                            }
                        }
                    }
                };
                #[doc(hidden)]
                #[allow(
                    non_upper_case_globals,
                    unused_attributes,
                    unused_qualifications,
                    clippy::absolute_paths,
                )]
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for AppListing {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private228::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __field1,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            #[automatically_derived]
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private228::Formatter,
                                ) -> _serde::__private228::fmt::Result {
                                    _serde::__private228::Formatter::write_str(
                                        __formatter,
                                        "variant identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private228::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private228::Ok(__Field::__field0),
                                        1u64 => _serde::__private228::Ok(__Field::__field1),
                                        _ => {
                                            _serde::__private228::Err(
                                                _serde::de::Error::invalid_value(
                                                    _serde::de::Unexpected::Unsigned(__value),
                                                    &"variant index 0 <= i < 2",
                                                ),
                                            )
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private228::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "Full" => _serde::__private228::Ok(__Field::__field0),
                                        "Unknown" => _serde::__private228::Ok(__Field::__field1),
                                        _ => {
                                            _serde::__private228::Err(
                                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                                            )
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private228::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"Full" => _serde::__private228::Ok(__Field::__field0),
                                        b"Unknown" => _serde::__private228::Ok(__Field::__field1),
                                        _ => {
                                            let __value = &_serde::__private228::from_utf8_lossy(
                                                __value,
                                            );
                                            _serde::__private228::Err(
                                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                                            )
                                        }
                                    }
                                }
                            }
                            #[automatically_derived]
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private228::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            const VARIANTS: &'static [&'static str] = &[
                                "Full",
                                "Unknown",
                            ];
                            #[doc(hidden)]
                            struct __Seed<'de> {
                                variant: __Field,
                                marker: _serde::__private228::PhantomData<AppListing>,
                                lifetime: _serde::__private228::PhantomData<&'de ()>,
                            }
                            #[automatically_derived]
                            impl<'de> _serde::de::DeserializeSeed<'de> for __Seed<'de> {
                                type Value = AppListing;
                                fn deserialize<__D>(
                                    self,
                                    __deserializer: __D,
                                ) -> _serde::__private228::Result<Self::Value, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    match self.variant {
                                        __Field::__field0 => {
                                            #[allow(non_camel_case_types)]
                                            #[doc(hidden)]
                                            enum __Field {
                                                __field0,
                                                __ignore,
                                            }
                                            #[doc(hidden)]
                                            struct __FieldVisitor;
                                            #[automatically_derived]
                                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                                type Value = __Field;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private228::Formatter,
                                                ) -> _serde::__private228::fmt::Result {
                                                    _serde::__private228::Formatter::write_str(
                                                        __formatter,
                                                        "field identifier",
                                                    )
                                                }
                                                fn visit_u64<__E>(
                                                    self,
                                                    __value: u64,
                                                ) -> _serde::__private228::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        0u64 => _serde::__private228::Ok(__Field::__field0),
                                                        _ => _serde::__private228::Ok(__Field::__ignore),
                                                    }
                                                }
                                                fn visit_str<__E>(
                                                    self,
                                                    __value: &str,
                                                ) -> _serde::__private228::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        "app_id" => _serde::__private228::Ok(__Field::__field0),
                                                        _ => _serde::__private228::Ok(__Field::__ignore),
                                                    }
                                                }
                                                fn visit_bytes<__E>(
                                                    self,
                                                    __value: &[u8],
                                                ) -> _serde::__private228::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        b"app_id" => _serde::__private228::Ok(__Field::__field0),
                                                        _ => _serde::__private228::Ok(__Field::__ignore),
                                                    }
                                                }
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::Deserialize<'de> for __Field {
                                                #[inline]
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private228::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::Deserializer::deserialize_identifier(
                                                        __deserializer,
                                                        __FieldVisitor,
                                                    )
                                                }
                                            }
                                            #[doc(hidden)]
                                            struct __Visitor<'de> {
                                                marker: _serde::__private228::PhantomData<AppListing>,
                                                lifetime: _serde::__private228::PhantomData<&'de ()>,
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = AppListing;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private228::Formatter,
                                                ) -> _serde::__private228::fmt::Result {
                                                    _serde::__private228::Formatter::write_str(
                                                        __formatter,
                                                        "struct variant AppListing::Full",
                                                    )
                                                }
                                                #[inline]
                                                fn visit_map<__A>(
                                                    self,
                                                    mut __map: __A,
                                                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                                                where
                                                    __A: _serde::de::MapAccess<'de>,
                                                {
                                                    let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                                                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                                        __Field,
                                                    >(&mut __map)? {
                                                        match __key {
                                                            __Field::__field0 => {
                                                                if _serde::__private228::Option::is_some(&__field0) {
                                                                    return _serde::__private228::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field("app_id"),
                                                                    );
                                                                }
                                                                __field0 = _serde::__private228::Some(
                                                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                                );
                                                            }
                                                            _ => {
                                                                let _ = _serde::de::MapAccess::next_value::<
                                                                    _serde::de::IgnoredAny,
                                                                >(&mut __map)?;
                                                            }
                                                        }
                                                    }
                                                    let __field0 = match __field0 {
                                                        _serde::__private228::Some(__field0) => __field0,
                                                        _serde::__private228::None => {
                                                            _serde::__private228::de::missing_field("app_id")?
                                                        }
                                                    };
                                                    _serde::__private228::Ok(AppListing::Full {
                                                        app_id: __field0,
                                                    })
                                                }
                                            }
                                            #[doc(hidden)]
                                            const FIELDS: &'static [&'static str] = &["app_id"];
                                            _serde::Deserializer::deserialize_any(
                                                __deserializer,
                                                __Visitor {
                                                    marker: _serde::__private228::PhantomData::<AppListing>,
                                                    lifetime: _serde::__private228::PhantomData,
                                                },
                                            )
                                        }
                                        __Field::__field1 => {
                                            #[allow(non_camel_case_types)]
                                            #[doc(hidden)]
                                            enum __Field {
                                                __field0,
                                                __ignore,
                                            }
                                            #[doc(hidden)]
                                            struct __FieldVisitor;
                                            #[automatically_derived]
                                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                                type Value = __Field;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private228::Formatter,
                                                ) -> _serde::__private228::fmt::Result {
                                                    _serde::__private228::Formatter::write_str(
                                                        __formatter,
                                                        "field identifier",
                                                    )
                                                }
                                                fn visit_u64<__E>(
                                                    self,
                                                    __value: u64,
                                                ) -> _serde::__private228::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        0u64 => _serde::__private228::Ok(__Field::__field0),
                                                        _ => _serde::__private228::Ok(__Field::__ignore),
                                                    }
                                                }
                                                fn visit_str<__E>(
                                                    self,
                                                    __value: &str,
                                                ) -> _serde::__private228::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        "app_id" => _serde::__private228::Ok(__Field::__field0),
                                                        _ => _serde::__private228::Ok(__Field::__ignore),
                                                    }
                                                }
                                                fn visit_bytes<__E>(
                                                    self,
                                                    __value: &[u8],
                                                ) -> _serde::__private228::Result<Self::Value, __E>
                                                where
                                                    __E: _serde::de::Error,
                                                {
                                                    match __value {
                                                        b"app_id" => _serde::__private228::Ok(__Field::__field0),
                                                        _ => _serde::__private228::Ok(__Field::__ignore),
                                                    }
                                                }
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::Deserialize<'de> for __Field {
                                                #[inline]
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private228::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::Deserializer::deserialize_identifier(
                                                        __deserializer,
                                                        __FieldVisitor,
                                                    )
                                                }
                                            }
                                            #[doc(hidden)]
                                            struct __Visitor<'de> {
                                                marker: _serde::__private228::PhantomData<AppListing>,
                                                lifetime: _serde::__private228::PhantomData<&'de ()>,
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                                type Value = AppListing;
                                                fn expecting(
                                                    &self,
                                                    __formatter: &mut _serde::__private228::Formatter,
                                                ) -> _serde::__private228::fmt::Result {
                                                    _serde::__private228::Formatter::write_str(
                                                        __formatter,
                                                        "struct variant AppListing::Unknown",
                                                    )
                                                }
                                                #[inline]
                                                fn visit_map<__A>(
                                                    self,
                                                    mut __map: __A,
                                                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                                                where
                                                    __A: _serde::de::MapAccess<'de>,
                                                {
                                                    let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                                                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                                        __Field,
                                                    >(&mut __map)? {
                                                        match __key {
                                                            __Field::__field0 => {
                                                                if _serde::__private228::Option::is_some(&__field0) {
                                                                    return _serde::__private228::Err(
                                                                        <__A::Error as _serde::de::Error>::duplicate_field("app_id"),
                                                                    );
                                                                }
                                                                __field0 = _serde::__private228::Some(
                                                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                                );
                                                            }
                                                            _ => {
                                                                let _ = _serde::de::MapAccess::next_value::<
                                                                    _serde::de::IgnoredAny,
                                                                >(&mut __map)?;
                                                            }
                                                        }
                                                    }
                                                    let __field0 = match __field0 {
                                                        _serde::__private228::Some(__field0) => __field0,
                                                        _serde::__private228::None => {
                                                            _serde::__private228::de::missing_field("app_id")?
                                                        }
                                                    };
                                                    _serde::__private228::Ok(AppListing::Unknown {
                                                        app_id: __field0,
                                                    })
                                                }
                                            }
                                            #[doc(hidden)]
                                            const FIELDS: &'static [&'static str] = &["app_id"];
                                            _serde::Deserializer::deserialize_any(
                                                __deserializer,
                                                __Visitor {
                                                    marker: _serde::__private228::PhantomData::<AppListing>,
                                                    lifetime: _serde::__private228::PhantomData,
                                                },
                                            )
                                        }
                                    }
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: _serde::__private228::PhantomData<AppListing>,
                                lifetime: _serde::__private228::PhantomData<&'de ()>,
                            }
                            #[automatically_derived]
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = AppListing;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private228::Formatter,
                                ) -> _serde::__private228::fmt::Result {
                                    _serde::__private228::Formatter::write_str(
                                        __formatter,
                                        "adjacently tagged enum AppListing",
                                    )
                                }
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    match {
                                        let mut __rk: _serde::__private228::Option<
                                            _serde::__private228::de::TagOrContentField,
                                        > = _serde::__private228::None;
                                        while let _serde::__private228::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                            &mut __map,
                                            _serde::__private228::de::TagContentOtherFieldVisitor {
                                                tag: "event",
                                                content: "data",
                                            },
                                        )? {
                                            match __k {
                                                _serde::__private228::de::TagContentOtherField::Other => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                    continue;
                                                }
                                                _serde::__private228::de::TagContentOtherField::Tag => {
                                                    __rk = _serde::__private228::Some(
                                                        _serde::__private228::de::TagOrContentField::Tag,
                                                    );
                                                    break;
                                                }
                                                _serde::__private228::de::TagContentOtherField::Content => {
                                                    __rk = _serde::__private228::Some(
                                                        _serde::__private228::de::TagOrContentField::Content,
                                                    );
                                                    break;
                                                }
                                            }
                                        }
                                        __rk
                                    } {
                                        _serde::__private228::Some(
                                            _serde::__private228::de::TagOrContentField::Tag,
                                        ) => {
                                            let __field = _serde::de::MapAccess::next_value_seed(
                                                &mut __map,
                                                _serde::__private228::de::AdjacentlyTaggedEnumVariantSeed::<
                                                    __Field,
                                                > {
                                                    enum_name: "AppListing",
                                                    variants: VARIANTS,
                                                    fields_enum: _serde::__private228::PhantomData,
                                                },
                                            )?;
                                            match {
                                                let mut __rk: _serde::__private228::Option<
                                                    _serde::__private228::de::TagOrContentField,
                                                > = _serde::__private228::None;
                                                while let _serde::__private228::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                                    &mut __map,
                                                    _serde::__private228::de::TagContentOtherFieldVisitor {
                                                        tag: "event",
                                                        content: "data",
                                                    },
                                                )? {
                                                    match __k {
                                                        _serde::__private228::de::TagContentOtherField::Other => {
                                                            let _ = _serde::de::MapAccess::next_value::<
                                                                _serde::de::IgnoredAny,
                                                            >(&mut __map)?;
                                                            continue;
                                                        }
                                                        _serde::__private228::de::TagContentOtherField::Tag => {
                                                            __rk = _serde::__private228::Some(
                                                                _serde::__private228::de::TagOrContentField::Tag,
                                                            );
                                                            break;
                                                        }
                                                        _serde::__private228::de::TagContentOtherField::Content => {
                                                            __rk = _serde::__private228::Some(
                                                                _serde::__private228::de::TagOrContentField::Content,
                                                            );
                                                            break;
                                                        }
                                                    }
                                                }
                                                __rk
                                            } {
                                                _serde::__private228::Some(
                                                    _serde::__private228::de::TagOrContentField::Tag,
                                                ) => {
                                                    _serde::__private228::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("event"),
                                                    )
                                                }
                                                _serde::__private228::Some(
                                                    _serde::__private228::de::TagOrContentField::Content,
                                                ) => {
                                                    let __ret = _serde::de::MapAccess::next_value_seed(
                                                        &mut __map,
                                                        __Seed {
                                                            variant: __field,
                                                            marker: _serde::__private228::PhantomData,
                                                            lifetime: _serde::__private228::PhantomData,
                                                        },
                                                    )?;
                                                    match {
                                                        let mut __rk: _serde::__private228::Option<
                                                            _serde::__private228::de::TagOrContentField,
                                                        > = _serde::__private228::None;
                                                        while let _serde::__private228::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                                            &mut __map,
                                                            _serde::__private228::de::TagContentOtherFieldVisitor {
                                                                tag: "event",
                                                                content: "data",
                                                            },
                                                        )? {
                                                            match __k {
                                                                _serde::__private228::de::TagContentOtherField::Other => {
                                                                    let _ = _serde::de::MapAccess::next_value::<
                                                                        _serde::de::IgnoredAny,
                                                                    >(&mut __map)?;
                                                                    continue;
                                                                }
                                                                _serde::__private228::de::TagContentOtherField::Tag => {
                                                                    __rk = _serde::__private228::Some(
                                                                        _serde::__private228::de::TagOrContentField::Tag,
                                                                    );
                                                                    break;
                                                                }
                                                                _serde::__private228::de::TagContentOtherField::Content => {
                                                                    __rk = _serde::__private228::Some(
                                                                        _serde::__private228::de::TagOrContentField::Content,
                                                                    );
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                        __rk
                                                    } {
                                                        _serde::__private228::Some(
                                                            _serde::__private228::de::TagOrContentField::Tag,
                                                        ) => {
                                                            _serde::__private228::Err(
                                                                <__A::Error as _serde::de::Error>::duplicate_field("event"),
                                                            )
                                                        }
                                                        _serde::__private228::Some(
                                                            _serde::__private228::de::TagOrContentField::Content,
                                                        ) => {
                                                            _serde::__private228::Err(
                                                                <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                                            )
                                                        }
                                                        _serde::__private228::None => {
                                                            _serde::__private228::Ok(__ret)
                                                        }
                                                    }
                                                }
                                                _serde::__private228::None => {
                                                    _serde::__private228::Err(
                                                        <__A::Error as _serde::de::Error>::missing_field("data"),
                                                    )
                                                }
                                            }
                                        }
                                        _serde::__private228::Some(
                                            _serde::__private228::de::TagOrContentField::Content,
                                        ) => {
                                            let __content = _serde::de::MapAccess::next_value_seed(
                                                &mut __map,
                                                _serde::__private228::de::ContentVisitor::new(),
                                            )?;
                                            match {
                                                let mut __rk: _serde::__private228::Option<
                                                    _serde::__private228::de::TagOrContentField,
                                                > = _serde::__private228::None;
                                                while let _serde::__private228::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                                    &mut __map,
                                                    _serde::__private228::de::TagContentOtherFieldVisitor {
                                                        tag: "event",
                                                        content: "data",
                                                    },
                                                )? {
                                                    match __k {
                                                        _serde::__private228::de::TagContentOtherField::Other => {
                                                            let _ = _serde::de::MapAccess::next_value::<
                                                                _serde::de::IgnoredAny,
                                                            >(&mut __map)?;
                                                            continue;
                                                        }
                                                        _serde::__private228::de::TagContentOtherField::Tag => {
                                                            __rk = _serde::__private228::Some(
                                                                _serde::__private228::de::TagOrContentField::Tag,
                                                            );
                                                            break;
                                                        }
                                                        _serde::__private228::de::TagContentOtherField::Content => {
                                                            __rk = _serde::__private228::Some(
                                                                _serde::__private228::de::TagOrContentField::Content,
                                                            );
                                                            break;
                                                        }
                                                    }
                                                }
                                                __rk
                                            } {
                                                _serde::__private228::Some(
                                                    _serde::__private228::de::TagOrContentField::Tag,
                                                ) => {
                                                    let __seed = __Seed {
                                                        variant: _serde::de::MapAccess::next_value_seed(
                                                            &mut __map,
                                                            _serde::__private228::de::AdjacentlyTaggedEnumVariantSeed::<
                                                                __Field,
                                                            > {
                                                                enum_name: "AppListing",
                                                                variants: VARIANTS,
                                                                fields_enum: _serde::__private228::PhantomData,
                                                            },
                                                        )?,
                                                        marker: _serde::__private228::PhantomData,
                                                        lifetime: _serde::__private228::PhantomData,
                                                    };
                                                    let __deserializer = _serde::__private228::de::ContentDeserializer::<
                                                        __A::Error,
                                                    >::new(__content);
                                                    let __ret = _serde::de::DeserializeSeed::deserialize(
                                                        __seed,
                                                        __deserializer,
                                                    )?;
                                                    match {
                                                        let mut __rk: _serde::__private228::Option<
                                                            _serde::__private228::de::TagOrContentField,
                                                        > = _serde::__private228::None;
                                                        while let _serde::__private228::Some(__k) = _serde::de::MapAccess::next_key_seed(
                                                            &mut __map,
                                                            _serde::__private228::de::TagContentOtherFieldVisitor {
                                                                tag: "event",
                                                                content: "data",
                                                            },
                                                        )? {
                                                            match __k {
                                                                _serde::__private228::de::TagContentOtherField::Other => {
                                                                    let _ = _serde::de::MapAccess::next_value::<
                                                                        _serde::de::IgnoredAny,
                                                                    >(&mut __map)?;
                                                                    continue;
                                                                }
                                                                _serde::__private228::de::TagContentOtherField::Tag => {
                                                                    __rk = _serde::__private228::Some(
                                                                        _serde::__private228::de::TagOrContentField::Tag,
                                                                    );
                                                                    break;
                                                                }
                                                                _serde::__private228::de::TagContentOtherField::Content => {
                                                                    __rk = _serde::__private228::Some(
                                                                        _serde::__private228::de::TagOrContentField::Content,
                                                                    );
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                        __rk
                                                    } {
                                                        _serde::__private228::Some(
                                                            _serde::__private228::de::TagOrContentField::Tag,
                                                        ) => {
                                                            _serde::__private228::Err(
                                                                <__A::Error as _serde::de::Error>::duplicate_field("event"),
                                                            )
                                                        }
                                                        _serde::__private228::Some(
                                                            _serde::__private228::de::TagOrContentField::Content,
                                                        ) => {
                                                            _serde::__private228::Err(
                                                                <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                                            )
                                                        }
                                                        _serde::__private228::None => {
                                                            _serde::__private228::Ok(__ret)
                                                        }
                                                    }
                                                }
                                                _serde::__private228::Some(
                                                    _serde::__private228::de::TagOrContentField::Content,
                                                ) => {
                                                    _serde::__private228::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                                    )
                                                }
                                                _serde::__private228::None => {
                                                    _serde::__private228::Err(
                                                        <__A::Error as _serde::de::Error>::missing_field("event"),
                                                    )
                                                }
                                            }
                                        }
                                        _serde::__private228::None => {
                                            _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::missing_field("event"),
                                            )
                                        }
                                    }
                                }
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    match _serde::de::SeqAccess::next_element(&mut __seq)? {
                                        _serde::__private228::Some(__variant) => {
                                            match _serde::de::SeqAccess::next_element_seed(
                                                &mut __seq,
                                                __Seed {
                                                    variant: __variant,
                                                    marker: _serde::__private228::PhantomData,
                                                    lifetime: _serde::__private228::PhantomData,
                                                },
                                            )? {
                                                _serde::__private228::Some(__ret) => {
                                                    _serde::__private228::Ok(__ret)
                                                }
                                                _serde::__private228::None => {
                                                    _serde::__private228::Err(
                                                        _serde::de::Error::invalid_length(1, &self),
                                                    )
                                                }
                                            }
                                        }
                                        _serde::__private228::None => {
                                            _serde::__private228::Err(
                                                _serde::de::Error::invalid_length(0, &self),
                                            )
                                        }
                                    }
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["event", "data"];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                "AppListing",
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private228::PhantomData::<AppListing>,
                                    lifetime: _serde::__private228::PhantomData,
                                },
                            )
                        }
                    }
                };
                pub async fn _inner_list_apps(
                    dir: &str,
                    create_if_needed: bool,
                ) -> Option<Vec<AppListing>> {
                    if fs::read_dir(&dir).await.is_err() {
                        fs::create_dir_all(&dir).await.ok()?;
                    }
                    let mut tasks = ::alloc::vec::Vec::new();
                    let mut dir = fs::read_dir(&dir).await.ok()?;
                    while let Some(entry) = dir.next_entry().await.ok()? {
                        tasks.push(async move { Result::<(), String>::Ok(()) });
                    }
                    let val = join_all(tasks).await;
                    Some(
                        val
                            .into_iter()
                            .map(|x| match x {
                                Ok(()) => {
                                    AppListing::Full {
                                        app_id: "()".into(),
                                    }
                                }
                                Err(app_id) => AppListing::Unknown { app_id },
                            })
                            .collect::<Vec<_>>(),
                    )
                }
            }
            pub enum ConnectionType {
                Disconnected,
                Metered,
                Unmetered,
            }
            pub fn downloads<R: Runtime>(handle: &AppHandle<R>) -> Result<PathBuf> {
                let mut dir = handle.path().app_cache_dir()?;
                dir.push("data");
                _ = fs::create_dir_all(&dir);
                Ok(dir)
            }
            pub fn dwnl_tmp<R: Runtime>(handle: &AppHandle<R>) -> Result<PathBuf> {
                let mut dir = handle.path().app_cache_dir()?;
                dir.push("tmp");
                _ = fs::create_dir_all(&dir);
                Ok(dir)
            }
            pub fn clear_dwnl(handle: &AppHandle) {
                if let Some(dwn) = downloads(handle).ok() {
                    _ = fs::remove_dir_all(&dwn);
                }
                if let Some(tmp) = dwnl_tmp(handle).ok() {
                    _ = fs::remove_dir_all(&tmp);
                }
            }
        }
        pub mod downloader {
            use std::{
                io::SeekFrom, sync::{Arc, LazyLock},
                time::Duration,
            };
            use reqwest::{Client, ClientBuilder, StatusCode};
            use tauri::async_runtime::{self, JoinHandle};
            use tokio::{
                fs::{create_dir_all, remove_dir_all, remove_file, File, OpenOptions},
                io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
                sync::mpsc::{channel, error::TryRecvError, Sender},
                time::sleep,
            };
            static CLIENT: LazyLock<Client> = LazyLock::new(|| {
                ClientBuilder::new()
                    .user_agent(
                        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36",
                    )
                    .build()
                    .unwrap()
            });
            pub async fn download<F: FnMut(f64) -> (), T: FnOnce(u64) -> ()>(
                url: &str,
                out_file: &str,
                out_dir: &str,
                temp_dir: &str,
                turbo: bool,
                mut len_fn: T,
                mut log: F,
            ) -> Option<()> {
                let _ = remove_dir_all(out_dir).await;
                create_dir_all(out_dir).await.ok()?;
                let _ = remove_dir_all(temp_dir).await;
                create_dir_all(temp_dir).await.ok()?;
                let ranged = turbo && supports_ranged(url).await;
                log(0.0);
                let size = get_size(url).await;
                if ranged && size.is_some() {
                    dwn_ranged(
                            size,
                            url,
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("{0}/{1}", out_dir, out_file),
                                )
                            }),
                            temp_dir,
                            len_fn,
                            log,
                        )
                        .await?;
                } else {
                    let mut file = File::create(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("{0}/{1}", out_dir, out_file),
                                )
                            }),
                        )
                        .await
                        .ok()?;
                    let mut resp = CLIENT.get(url).send().await.ok()?;
                    let len = resp.content_length().unwrap_or(1);
                    len_fn(len);
                    let mut curr = 0u64;
                    let mut least = 0.0;
                    while let Some(x) = resp.chunk().await.ok()? {
                        curr += x.len() as u64;
                        file.write_all(&x).await.ok()?;
                        let perc = (curr as f64 * 100.0) / (len as f64);
                        if perc > (least + 0.5) || perc == 100.0 {
                            log(perc);
                            least = perc;
                        }
                    }
                    file.flush().await.ok()?;
                    drop(file);
                }
                Some(())
            }
            async fn dwn_ranged<F: FnMut(f64) -> (), T: FnOnce(u64) -> ()>(
                size: Option<u64>,
                url: &str,
                out: String,
                temp: &str,
                mut len: T,
                mut log: F,
            ) -> Option<()> {
                let mut file = File::create(&out).await.ok()?;
                let url = Arc::new(url.to_string());
                let temp = Arc::new(temp.to_string());
                let size = size?;
                let mut done = 0u64;
                len(size);
                let (tx, mut rx) = channel::<u64>(20);
                let mut pool = ::alloc::vec::Vec::new();
                for (start, end) in divide_into_ranges(size) {
                    let tx = tx.clone();
                    let url = url.clone();
                    let temp = temp.clone();
                    pool.push(
                        async_runtime::spawn(async move {
                            download_range(&url, tx, start, end, &temp).await
                        }),
                    );
                    sleep(Duration::from_micros(50)).await;
                }
                let mut last_prog: f64 = 0.0;
                log(0.0);
                loop {
                    while let Ok(x) = rx.try_recv() {
                        done += x;
                    }
                    match rx.try_recv() {
                        Err(e) => {
                            match e {
                                TryRecvError::Empty => {}
                                TryRecvError::Disconnected => {
                                    break;
                                }
                            }
                        }
                        Ok(x) => {
                            done += x;
                        }
                    }
                    let prog = (done as f64 * 100.0) / size as f64;
                    if prog != (last_prog + 0.5) || prog == 100.0 {
                        last_prog = prog;
                        log(prog);
                    }
                    if pool
                        .iter()
                        .all(|x| match x {
                            JoinHandle::Tokio(x) => x.is_finished(),
                        })
                    {
                        break;
                    }
                    sleep(Duration::from_millis(100)).await;
                }
                log(100.0);
                for data in pool {
                    let (path, mut data) = data.await.ok()??;
                    data.seek(SeekFrom::Start(0)).await.ok()?;
                    let mut buf = [0; 4096];
                    loop {
                        let size = data.read(&mut buf).await.ok()?;
                        if size == 0 {
                            break;
                        }
                        file.write(&buf[0..size]).await.ok()?;
                    }
                    drop(data);
                    remove_file(path).await.ok()?;
                }
                drop(file);
                Some(())
            }
            async fn download_range(
                url: &str,
                tx: Sender<u64>,
                start: u64,
                end: u64,
                temp: &str,
            ) -> Option<(String, File)> {
                let mut resp = CLIENT
                    .get(url)
                    .header(
                        "Range",
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("bytes={0}-{1}", start, end),
                            )
                        }),
                    )
                    .send()
                    .await
                    .ok()?;
                let file = ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}/{1}_to_{2}", temp, start, end),
                    )
                });
                let mut buf = OpenOptions::new();
                buf.create_new(true).read(true).write(true).truncate(true);
                buf.share_mode(0);
                let mut buf = buf.open(&file).await.ok()?;
                while let Some(chunk) = resp.chunk().await.ok()? {
                    let _ = tx.send(chunk.len() as u64).await;
                    buf.write_all(&chunk).await.ok()?;
                }
                Some((file, buf))
            }
            fn divide_into_ranges(total_size: u64) -> Vec<(u64, u64)> {
                let mut total = total_size / 1024 * 1024;
                if total > 100 {
                    total = 100;
                }
                let chunk_size = total_size / total;
                let mut ranges = Vec::new();
                for i in 0..total {
                    let start = i * chunk_size;
                    let end = if i == (total - 1) {
                        total_size - 1
                    } else {
                        start + chunk_size - 1
                    };
                    ranges.push((start, end));
                }
                ranges
            }
            pub async fn get_size(url: &str) -> Option<u64> {
                let resp = CLIENT.head(url).send().await.ok()?;
                let headers = resp.headers();
                headers
                    .get("content-length")
                    .map_or(headers.get("Content-Length"), |x| Some(x))
                    .map_or(None, |x| x.to_str().ok()?.parse().ok())
            }
            async fn supports_ranged(url: &str) -> bool {
                let res = CLIENT.get(url).header("Range", "bytes=0-0").send().await;
                if let Ok(res) = res {
                    return res.status() == StatusCode::PARTIAL_CONTENT;
                }
                false
            }
        }
        pub mod windows {
            use tauri::AppHandle;
            use ahqstore_types::{AHQStoreApplication, DownloadUrl};
            pub mod notify {
                use std::{sync::LazyLock, thread::sleep, time::Duration};
                use win32_notif::{
                    ManageNotification, NotificationActivatedEventHandler,
                    NotificationBuilder, NotificationDataSet, ToastsNotifier,
                    notification::{
                        AdaptiveText, Scenario,
                        actions::{ActionButton, action::ActivationType},
                        audio::{Audio, Src},
                        group::{Group, SubGroup},
                        visual::{
                            image::{AdaptiveImageAlign, ImageCrop},
                            progress::ProgressValue, text::HintStyle, *,
                        },
                    },
                };
                pub static NOTIF: LazyLock<ToastsNotifier> = LazyLock::new(|| {
                    ToastsNotifier::new("Microsoft.Windows.Explorer")
                        .expect("Unexpected error while trying to create notifier")
                });
                pub(crate) fn send_update(total: u16) {
                    let updated = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} apps will be updated", total),
                        )
                    });
                    let notif = NotificationBuilder::new()
                        .with_scenario(Scenario::Reminder)
                        .audio(Audio::new(Src::Reminder, false, false))
                        .visual(
                            Text::create(0, "Updates Available")
                                .with_style(HintStyle::Title),
                        )
                        .visual(
                            Text::create(
                                    0,
                                    "Application updates are available, would you like to update?",
                                )
                                .with_style(HintStyle::Subtitle),
                        )
                        .visual(
                            Group::new()
                                .with_subgroup(
                                    SubGroup::new()
                                        .with_visual(
                                            Text::create(0, &updated).with_style(HintStyle::Base),
                                        )
                                        .with_visual(
                                            Text::create(0, "Only unattended updates will be processed")
                                                .with_style(HintStyle::CaptionSubtle),
                                        ),
                                ),
                        )
                        .action(ActionButton::create("Yes").with_id("update"))
                        .action(ActionButton::create("No").with_id("no"))
                        .on_activated(
                            NotificationActivatedEventHandler::new(|_a, b| {
                                let notif = b.unwrap();
                                if &notif.button_id.unwrap_or_default() == "update" {
                                    {
                                        ::core::panicking::panic_fmt(format_args!("Called update"));
                                    };
                                }
                                Ok(())
                            }),
                        )
                        .with_use_button_style(true)
                        .build(0, &NOTIF, "2", "1")
                        .unwrap();
                    _ = notif.set_expires_on_reboot(true);
                    notif.show().unwrap();
                }
                pub(crate) fn app_id_updating(src: &str, app_id: &str, status: &str) {
                    let notif = NotificationBuilder::new()
                        .with_scenario(Scenario::Default)
                        .audio(Audio::new(Src::IM, false, false))
                        .visual(
                            Text::create_binded(0, "title").with_style(HintStyle::Title),
                        )
                        .visual(
                            Text::create_binded(0, "body").with_style(HintStyle::Body),
                        )
                        .visual(
                            Image::create(1, src)
                                .with_align(AdaptiveImageAlign::Default)
                                .with_crop(ImageCrop::Circle)
                                .with_placement(Placement::AppLogoOverride),
                        )
                        .visual(
                            Progress::create(
                                    AdaptiveText::BindTo("txt"),
                                    ProgressValue::BindTo("prog"),
                                )
                                .with_override_value(AdaptiveText::BindTo("txxt")),
                        )
                        .value("title", status)
                        .value("body", "Please wait while we update it for you")
                        .value("txt", "Downloading")
                        .value("prog", "indeterminate")
                        .value("txxt", "Starting up...")
                        .with_use_button_style(true)
                        .build(0, &NOTIF, app_id, "update")
                        .unwrap();
                    _ = notif.set_expires_on_reboot(true);
                    notif.show().unwrap();
                }
                pub(crate) fn updates_pending(apps: u16) {
                    let pending = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} apps have updates pending", apps),
                        )
                    });
                    let notif = NotificationBuilder::new()
                        .with_scenario(Scenario::Reminder)
                        .audio(Audio::new(Src::Reminder, false, false))
                        .visual(
                            Text::create(0, "Updates Pending")
                                .with_style(HintStyle::Title),
                        )
                        .visual(
                            Text::create(
                                    0,
                                    "A few updates are pending that require your attention.",
                                )
                                .with_style(HintStyle::Subtitle),
                        )
                        .visual(
                            Group::new()
                                .with_subgroup(
                                    SubGroup::new()
                                        .with_visual(
                                            Text::create(0, "{x}").with_style(HintStyle::Base),
                                        )
                                        .with_visual(
                                            Text::create(0, "These require your action to update")
                                                .with_style(HintStyle::CaptionSubtle),
                                        ),
                                ),
                        )
                        .value("x", &pending)
                        .action(
                            ActionButton::create("Launch store")
                                .with_activation_type(ActivationType::Protocol)
                                .with_id("launch"),
                        )
                        .on_activated(
                            NotificationActivatedEventHandler::new(|_a, b| {
                                let notif = b.unwrap();
                                {
                                    ::std::io::_print(format_args!("{0:#?}\n", notif));
                                };
                                Ok(())
                            }),
                        )
                        .with_use_button_style(true)
                        .build(0, &NOTIF, "2", "1")
                        .unwrap();
                    _ = notif.set_expires_on_reboot(true);
                    notif.show().unwrap();
                }
            }
            pub mod install {
                use std::sync::LazyLock;
                use std::env;
                use tauri::fs;
                pub(crate) static AHQSTORE_GLOBAL_DIR: LazyLock<String> = LazyLock::new(||
                {
                    let root = env::var("SYSTEMDRIVE").expect("Impossible error");
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{0}\\ProgramData\\AHQ Store Applications",
                                root,
                            ),
                        )
                    })
                });
                pub(crate) static AHQSTORE_USER_DIR: LazyLock<String> = LazyLock::new(|| {
                    let useroot = env::var("USERPROFILE").expect("Impossible error");
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}\\ahqstore", useroot))
                    })
                });
            }
            pub mod network {
                use windows::Networking::Connectivity::{
                    NetworkConnectivityLevel, NetworkCostType, NetworkInformation,
                };
                use crate::structs::platform::common::ConnectionType;
                pub fn get_network() -> Option<ConnectionType> {
                    let info = NetworkInformation::GetInternetConnectionProfile().ok()?;
                    let cost = info.GetConnectionCost().ok()?.NetworkCostType().ok()?;
                    let netlevel = info.GetNetworkConnectivityLevel().ok()?.0;
                    let conn = NetworkConnectivityLevel::InternetAccess.0 == netlevel;
                    if !conn {
                        return Some(ConnectionType::Disconnected);
                    }
                    let unmetered = cost.0 == NetworkCostType::Unrestricted.0;
                    if unmetered {
                        Some(ConnectionType::Unmetered)
                    } else {
                        Some(ConnectionType::Metered)
                    }
                }
            }
            pub fn is_supported(
                _: &AppHandle,
                app: &AHQStoreApplication,
            ) -> crate::Result<bool> {
                Ok(app.is_supported())
            }
            pub fn get_download(
                app: &AHQStoreApplication,
            ) -> (Option<&DownloadUrl>, Option<&'static str>) {
                (app.get_win_download(), app.get_win_extension())
            }
        }
        pub use windows as os;
        use crate::{
            structs::platform::common::{clear_dwnl, downloads, dwnl_tmp},
            AhqstoreExt, DownloadEvent, Error, InstallStat, Result,
        };
        #[allow(
            unreachable_code,
            clippy::diverging_sub_expression,
            clippy::used_underscore_binding
        )]
        const _: () = if false {
            #[diagnostic::on_unimplemented(
                message = "async commands that contain references as inputs must return a `Result`"
            )]
            trait AsyncCommandMustReturnResult {}
            impl<A, B> AsyncCommandMustReturnResult for ::std::result::Result<A, B> {}
            let _check: Result<Channel<()>> = ::core::panicking::panic(
                "internal error: entered unreachable code",
            );
            let _: &dyn AsyncCommandMustReturnResult = &_check;
        };
        pub async fn download(
            handle: AppHandle,
            app_id: &str,
            prog: Channel<DownloadEvent>,
        ) -> Result<Channel<()>> {
            let commit = &handle.ahqstore().commits.read().await.commit;
            let app = internet::get_app(commit, app_id).await?;
            let hwnd = handle.clone();
            let stat = tokio::spawn(async move {
                if !os::is_supported(&hwnd, &app)? {
                    return Err(Error::UnsupportedPlatform);
                }
                let (Some(url), Some(extension)) = os::get_download(&app) else {
                    return Err(Error::AHQStore(AnyError::msg("Could not get data")));
                };
                let turbo = false;
                let dwnl = downloads(&hwnd)?;
                let dir_tmp = dwnl_tmp(&hwnd)?;
                downloader::download(
                        &url.url,
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0}{1}", &app.appId, extension),
                            )
                        }),
                        { dwnl.to_str().context("Invalid String")? },
                        { dir_tmp.to_str().context("Invalid String")? },
                        turbo,
                        |length| {
                            prog.send(DownloadEvent::Started { length });
                        },
                        |progress| {
                            prog.send(DownloadEvent::Progress {
                                progress,
                            });
                        },
                    )
                    .await
                    .context("Unable to download")?;
                prog.send(DownloadEvent::Finished {});
                Ok(())
            });
            let app_id = app_id.to_string();
            let handle = handle.clone();
            let handler = Channel::new(move |body| {
                if !stat.is_finished() {
                    stat.abort();
                    {
                        clear_dwnl(&handle);
                    }
                    Ok(())
                } else {
                    Err(
                        tauri::Error::Anyhow(
                            AnyError::msg("The download task is already complete"),
                        ),
                    )
                }
            });
            Ok(handler)
        }
        #[allow(unused_imports)]
        pub use __cmd__download;
    }
    pub(crate) mod search {
        use std::{
            num::NonZero, sync::Arc, thread::available_parallelism, time::Duration,
        };
        use ahqstore_types::{Commits, SearchEntry};
        use tantivy::{
            collector::TopDocs,
            query::{BooleanQuery, BoostQuery, FuzzyTermQuery, Query, QueryParser},
            schema::{Field, Schema, Value, FAST, STRING, TEXT},
            Index, IndexReader, ReloadPolicy, TantivyDocument, Term,
        };
        use tauri::{async_runtime::spawn_blocking, AppHandle, Manager, Runtime};
        use tokio::{fs, sync::RwLock, time::sleep};
        use ahqstore_types::internet::get_all_search;
        pub struct CommitSearchIndex {
            pub commit: Commits,
            pub last_updated_secs: u64,
            pub meta: Option<SearchMeta>,
        }
        pub struct SearchMeta {
            pub reader: IndexReader,
            pub name_txt: Field,
            pub title_txt: Field,
            pub id: Field,
        }
        impl CommitSearchIndex {
            pub fn search(&self, query: &str) -> Result<Vec<String>, ()> {
                let Some(meta) = &self.meta else {
                    return Err(());
                };
                let searcher = meta.reader.searcher();
                let name_query = BoostQuery::new(
                    Box::new(
                        BooleanQuery::intersection(
                            query
                                .split_whitespace()
                                .into_iter()
                                .map(|x| {
                                    let x = Term::from_field_text(meta.name_txt, x);
                                    Box::new(FuzzyTermQuery::new(x, 1, true))
                                        as Box<dyn Query + 'static>
                                })
                                .collect::<Vec<Box<_>>>() as _,
                        ),
                    ),
                    1.5,
                );
                let title_query = BooleanQuery::intersection(
                    query
                        .split_whitespace()
                        .into_iter()
                        .map(|x| {
                            let x = Term::from_field_text(meta.title_txt, x);
                            Box::new(FuzzyTermQuery::new(x, 1, true))
                                as Box<dyn Query + 'static>
                        })
                        .collect::<Vec<Box<_>>>(),
                );
                let query = BooleanQuery::union(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Box::new(name_query),
                            Box::new(title_query),
                        ]),
                    ),
                );
                let Ok(mut search) = searcher.search(&query, &TopDocs::with_limit(500))
                else {
                    {
                        ::std::io::_print(format_args!("ERRORED OUT SEARCH\n"));
                    };
                    return Err(());
                };
                let results: Vec<String> = search
                    .into_iter()
                    .map(|(_, b)| {
                        searcher
                            .segment_reader(b.segment_ord)
                            .fast_fields()
                            .str("id")
                            .ok()
                            .flatten()
                            .and_then(|x| {
                                let mut out = [None; 1];
                                x.ords().first_vals(&[b.doc_id], &mut out);
                                let out = out[0]?;
                                let mut data = String::default();
                                x.ord_to_str(out, &mut data).ok()?;
                                Some(data)
                            })
                    })
                    .flatten()
                    .collect();
                Ok(results)
            }
        }
        pub async fn search_daemon<R: Runtime>(
            hwnd: AppHandle<R>,
            lck: Arc<RwLock<CommitSearchIndex>>,
        ) {
            let mut initial = lck.read().await.commit.clone();
            let mut builder = Schema::builder();
            let name_txt = builder.add_text_field("name", TEXT);
            let title_txt = builder.add_text_field("title", TEXT);
            let id = builder.add_text_field("id", STRING | FAST);
            let schema = builder.build();
            let index = {
                let mut cache = hwnd
                    .path()
                    .app_cache_dir()
                    .expect("Cannot fetch cache dir, exiting");
                cache.push("searchdb");
                _ = fs::create_dir_all(&cache).await;
                {
                    cache.push("commit.lck");
                    let data = fs::read_to_string(&cache).await.unwrap_or_default();
                    if &data
                        != &::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0:?}", initial))
                        })
                    {
                        initial.ahqstore = Default::default();
                        initial.alt = Default::default();
                    }
                    cache.pop();
                }
                cache.push("index");
                if cache.exists() && &initial.ahqstore != "" {
                    Index::open_in_dir(&cache).expect("Unable to create db, exiting")
                } else {
                    _ = fs::remove_dir_all(&cache).await;
                    _ = fs::create_dir_all(&cache).await;
                    Index::create_in_dir(&cache, schema)
                        .expect("Unable to create db, exiting")
                }
            };
            let index = Arc::new(index);
            {
                let mut data = lck.write().await;
                data.meta = Some(SearchMeta {
                    id,
                    name_txt,
                    reader: index
                        .reader_builder()
                        .reload_policy(ReloadPolicy::OnCommitWithDelay)
                        .num_warming_threads(
                            available_parallelism()
                                .unwrap_or(unsafe { NonZero::new_unchecked(4usize) })
                                .get()
                                .clamp(2, 4),
                        )
                        .try_into()
                        .expect("Unable to construct reader"),
                    title_txt,
                });
                drop(data);
            }
            loop {
                let mut to_update;
                {
                    let commit_lck = lck.read().await;
                    let commit = &commit_lck.commit;
                    if &initial == commit {
                        sleep(Duration::from_secs(30)).await;
                        continue;
                    }
                    to_update = commit.clone();
                    drop(commit_lck);
                }
                if let Ok(searches) = get_all_search(&to_update).await {
                    {
                        ::std::io::_print(format_args!("[INFO] Parsing search map\n"));
                    };
                    let idx_c = index.clone();
                    if let Ok(Ok(_)) = spawn_blocking(move || build_search(
                            idx_c.as_ref(),
                            name_txt,
                            title_txt,
                            id,
                            searches,
                        ))
                        .await
                    {
                        let mut wrlck = lck.write().await;
                        if &wrlck.commit == &to_update {
                            initial = to_update;
                            let mut cache = hwnd
                                .path()
                                .app_cache_dir()
                                .expect("Cannot fetch cache dir, exiting");
                            cache.push("searchdb");
                            cache.push("commit.lck");
                            _ = fs::write(
                                    cache,
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("{0:?}", initial))
                                    }),
                                )
                                .await;
                        }
                    }
                }
                sleep(Duration::from_secs(30)).await;
            }
        }
        fn build_search(
            index: &Index,
            name_txt: Field,
            title_txt: Field,
            id: Field,
            data: Vec<SearchEntry>,
        ) -> tantivy::Result<()> {
            let mut index_writer = index
                .writer(
                    data
                        .len()
                        .saturating_mul(128)
                        .clamp(15 * 1024 * 1024, 100 * 1024 * 1024),
                )?;
            index_writer.delete_all_documents()?;
            for item in data {
                index_writer
                    .add_document({
                        let mut document = ::tantivy::TantivyDocument::default();
                        document.add_field_value(name_txt, &item.name);
                        document.add_field_value(title_txt, &item.title);
                        document.add_field_value(id, &item.id);
                        document
                    })?;
            }
            index_writer.commit()?;
            index_writer.wait_merging_threads()?;
            Ok(())
        }
    }
    use search::CommitSearchIndex;
    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time is going in reverse")
            .as_secs()
    }
    pub fn init<R: Runtime, C: DeserializeOwned>(
        app: &AppHandle<R>,
        _api: PluginApi<R, C>,
    ) -> crate::Result<Ahqstore<R>> {
        let commits = Arc::new(
            RwLock::new(
                async_runtime::block_on(async {
                    Ok::<
                        CommitSearchIndex,
                        anyhow::Error,
                    >(CommitSearchIndex {
                        commit: get_all_commits(None).await?,
                        last_updated_secs: now(),
                        meta: None,
                    })
                })?,
            ),
        );
        let prefs = Preferences::init(app)?;
        Ok(Ahqstore {
            handle: app.clone(),
            commits,
            send_to_ipc: Mutex::new(None),
            preferences: Arc::new(RwLock::new(prefs)),
        })
    }
    pub enum AutoUpdate {
        Never,
        CheckOnly,
        UpdateDuringUnmeteredWifi,
        UpdateDuringMeteredWifi,
        Always,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AutoUpdate {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    AutoUpdate::Never => "Never",
                    AutoUpdate::CheckOnly => "CheckOnly",
                    AutoUpdate::UpdateDuringUnmeteredWifi => "UpdateDuringUnmeteredWifi",
                    AutoUpdate::UpdateDuringMeteredWifi => "UpdateDuringMeteredWifi",
                    AutoUpdate::Always => "Always",
                },
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for AutoUpdate {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    AutoUpdate::Never => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "AutoUpdate",
                            0u32,
                            "Never",
                        )
                    }
                    AutoUpdate::CheckOnly => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "AutoUpdate",
                            1u32,
                            "CheckOnly",
                        )
                    }
                    AutoUpdate::UpdateDuringUnmeteredWifi => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "AutoUpdate",
                            2u32,
                            "UpdateDuringUnmeteredWifi",
                        )
                    }
                    AutoUpdate::UpdateDuringMeteredWifi => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "AutoUpdate",
                            3u32,
                            "UpdateDuringMeteredWifi",
                        )
                    }
                    AutoUpdate::Always => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "AutoUpdate",
                            4u32,
                            "Always",
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for AutoUpdate {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            3u64 => _serde::__private228::Ok(__Field::__field3),
                            4u64 => _serde::__private228::Ok(__Field::__field4),
                            _ => {
                                _serde::__private228::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 5",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Never" => _serde::__private228::Ok(__Field::__field0),
                            "CheckOnly" => _serde::__private228::Ok(__Field::__field1),
                            "UpdateDuringUnmeteredWifi" => {
                                _serde::__private228::Ok(__Field::__field2)
                            }
                            "UpdateDuringMeteredWifi" => {
                                _serde::__private228::Ok(__Field::__field3)
                            }
                            "Always" => _serde::__private228::Ok(__Field::__field4),
                            _ => {
                                _serde::__private228::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Never" => _serde::__private228::Ok(__Field::__field0),
                            b"CheckOnly" => _serde::__private228::Ok(__Field::__field1),
                            b"UpdateDuringUnmeteredWifi" => {
                                _serde::__private228::Ok(__Field::__field2)
                            }
                            b"UpdateDuringMeteredWifi" => {
                                _serde::__private228::Ok(__Field::__field3)
                            }
                            b"Always" => _serde::__private228::Ok(__Field::__field4),
                            _ => {
                                let __value = &_serde::__private228::from_utf8_lossy(
                                    __value,
                                );
                                _serde::__private228::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<AutoUpdate>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AutoUpdate;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "enum AutoUpdate",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private228::Ok(AutoUpdate::Never)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private228::Ok(AutoUpdate::CheckOnly)
                            }
                            (__Field::__field2, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private228::Ok(
                                    AutoUpdate::UpdateDuringUnmeteredWifi,
                                )
                            }
                            (__Field::__field3, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private228::Ok(
                                    AutoUpdate::UpdateDuringMeteredWifi,
                                )
                            }
                            (__Field::__field4, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private228::Ok(AutoUpdate::Always)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "Never",
                    "CheckOnly",
                    "UpdateDuringUnmeteredWifi",
                    "UpdateDuringMeteredWifi",
                    "Always",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "AutoUpdate",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<AutoUpdate>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    pub struct Preferences {
        #[serde(rename = "autoUpdate")]
        pub auto_update: AutoUpdate,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Preferences {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Preferences",
                "auto_update",
                &&self.auto_update,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Preferences {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Preferences",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "autoUpdate",
                    &self.auto_update,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Preferences {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "autoUpdate" => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"autoUpdate" => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<Preferences>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Preferences;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct Preferences",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            AutoUpdate,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Preferences with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(Preferences {
                            auto_update: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<AutoUpdate> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "autoUpdate",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<AutoUpdate>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("autoUpdate")?
                            }
                        };
                        _serde::__private228::Ok(Preferences {
                            auto_update: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["autoUpdate"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Preferences",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Preferences>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    impl Preferences {
        pub fn init<R: Runtime>(h: &AppHandle<R>) -> crate::Result<Self> {
            use std::fs::read_to_string;
            use tauri::Manager;
            let mut set_path = h.path().app_local_data_dir()?;
            set_path.push("config.json");
            return Ok(
                serde_json::from_str(&read_to_string(&set_path).unwrap_or_default())
                    .unwrap_or(Self {
                        auto_update: AutoUpdate::CheckOnly,
                    }),
            );
        }
    }
    /// Access to the ahqstore APIs.
    pub struct Ahqstore<R: Runtime> {
        pub(crate) handle: AppHandle<R>,
        pub preferences: Arc<RwLock<Preferences>>,
        pub commits: Arc<RwLock<CommitSearchIndex>>,
        pub send_to_ipc: Mutex<Option<IPCSend>>,
    }
    impl<R: Runtime> Ahqstore<R> {
        pub fn init(&self, hwnd: AppHandle<R>) {
            let mut lock = self.send_to_ipc.blocking_lock();
            let hwnd2 = hwnd.clone();
            if lock.is_none() {
                *lock = Some(initialize(hwnd, self.commits.clone()));
            }
            let lck = self.commits.clone();
            spawn(async move {
                search_daemon(hwnd2, lck).await;
            });
        }
        pub async fn can_update_commit(&self) -> bool {
            let exp = self.commits.read().await.last_updated_secs;
            (exp + 60) < now()
        }
        pub async fn refresh(&self) -> crate::Result<()> {
            if !self.can_update_commit().await {
                use crate::Error;
                return Err(Error::CannotUpdate);
            }
            let mut lock = self.commits.write().await;
            *lock = CommitSearchIndex {
                commit: get_all_commits(None).await?,
                last_updated_secs: now(),
                meta: None,
            };
            Ok(())
        }
    }
}
mod commands {
    #![allow(unused)]
    use std::path::PathBuf;
    use std::time::SystemTime;
    use ahqstore_types::methods::OfficialManifestSource;
    use ahqstore_types::{AHQStoreApplication, Commits, DevData, Home};
    use anyhow::Context;
    use tauri::ipc::{IpcResponse, Response};
    use tauri::{command, AppHandle, Manager, Runtime};
    use crate::models::*;
    use crate::AhqstoreExt;
    use ahqstore_types::internet;
    use tauri::{
        window::{ProgressBarState, ProgressBarStatus},
        WebviewWindowBuilder,
    };
    use crate::error::Result;
    use open as open_2;
    mod download {
        use std::sync::Arc;
        use tauri::{ipc::Channel, AppHandle, Manager};
        use crate::{
            structs::platform::downloader::{self, get_size},
            DownloadEvent, Result,
        };
        use anyhow::Context;
    }
    mod encrypt {
        use crate::error::Result;
        use anyhow::Context;
        use chacha20poly1305::{
            aead::{generic_array::GenericArray, Aead, KeyInit},
            ChaCha20Poly1305,
        };
        use lazy_static::lazy_static;
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct CRYPTER {
            __private_field: (),
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        static CRYPTER: CRYPTER = CRYPTER { __private_field: () };
        impl ::lazy_static::__Deref for CRYPTER {
            type Target = ChaCha20Poly1305;
            fn deref(&self) -> &ChaCha20Poly1305 {
                #[inline(always)]
                fn __static_ref_initialize() -> ChaCha20Poly1305 {
                    {
                        let key = GenericArray::from_slice(
                            "$2b$10$hjtT7QV7UJcmMP6SLJGJVumGb".as_bytes(),
                        );
                        ChaCha20Poly1305::new(&key)
                    }
                }
                #[inline(always)]
                fn __stability() -> &'static ChaCha20Poly1305 {
                    static LAZY: ::lazy_static::lazy::Lazy<ChaCha20Poly1305> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for CRYPTER {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        static SALT: [u8; 16] = [
            0x14, 0x4b, 0x3d, 0x69, 0x1a, 0x7b, 0x4e, 0xcf, 0x39, 0xcf, 0x73, 0x5c, 0x7f,
            0xa7, 0xa7, 0x9c,
        ];
        use serde_json::to_string;
        pub fn encrypt(payload: String) -> Result<Vec<u8>> {
            let nonce = GenericArray::from_slice(b"SSSSSSSSSSSS");
            Ok(
                CRYPTER
                    .encrypt(nonce, payload.as_bytes())
                    .ok()
                    .context("Cannot encrypt")?,
            )
        }
        #[allow(unused_imports)]
        pub use __cmd__encrypt;
        pub fn decrypt(encrypted: Vec<u8>) -> Result<String> {
            let nonce = GenericArray::from_slice(b"SSSSSSSSSSSS");
            let decrypted = CRYPTER
                .decrypt(nonce, &*encrypted)
                .ok()
                .context("Cannot decrypt")?;
            Ok(String::from_utf8(decrypted)?)
        }
        #[allow(unused_imports)]
        pub use __cmd__decrypt;
    }
    pub use download::*;
    pub use encrypt::*;
    pub(crate) async fn get_commit<R: Runtime>(app: tauri::AppHandle<R>) -> Response {
        Response::new(
            serde_json::to_string(&app.ahqstore().commits.read().await.commit)
                .unwrap()
                .into_bytes(),
        )
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__get_commit;
    pub(crate) async fn set_scale(window: tauri::WebviewWindow, scale: f64) {
        let _ = window.set_zoom(scale);
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__set_scale;
    pub async fn to_hash_uid(id: String) -> String {
        ahqstore_gh_hash::compute(id)
    }
    #[allow(unused_imports)]
    pub use __cmd__to_hash_uid;
    pub(crate) async fn refresh_commit(app: AppHandle) {
        app.ahqstore().refresh().await;
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__refresh_commit;
    #[allow(
        unreachable_code,
        clippy::diverging_sub_expression,
        clippy::used_underscore_binding
    )]
    const _: () = if false {
        #[diagnostic::on_unimplemented(
            message = "async commands that contain references as inputs must return a `Result`"
        )]
        trait AsyncCommandMustReturnResult {}
        impl<A, B> AsyncCommandMustReturnResult for ::std::result::Result<A, B> {}
        let _check: Result<Vec<String>> = ::core::panicking::panic(
            "internal error: entered unreachable code",
        );
        let _: &dyn AsyncCommandMustReturnResult = &_check;
    };
    pub(crate) async fn get_all_search(
        app: AppHandle,
        query: &str,
    ) -> Result<Vec<String>> {
        Ok(
            app
                .ahqstore()
                .commits
                .read()
                .await
                .search(query)
                .map_err(|_| crate::Error::SearchError)?,
        )
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__get_all_search;
    pub(crate) async fn get_home(app: AppHandle) -> Result<Home> {
        Ok(
            internet::get_home(
                    (|| {
                        return OfficialManifestSource::WinGet;
                    })(),
                    &app.ahqstore().commits.read().await.commit.alt,
                )
                .await?,
        )
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__get_home;
    #[allow(
        unreachable_code,
        clippy::diverging_sub_expression,
        clippy::used_underscore_binding
    )]
    const _: () = if false {
        #[diagnostic::on_unimplemented(
            message = "async commands that contain references as inputs must return a `Result`"
        )]
        trait AsyncCommandMustReturnResult {}
        impl<A, B> AsyncCommandMustReturnResult for ::std::result::Result<A, B> {}
        let _check: Result<AHQStoreApplication> = ::core::panicking::panic(
            "internal error: entered unreachable code",
        );
        let _: &dyn AsyncCommandMustReturnResult = &_check;
    };
    pub(crate) async fn get_app(
        appl: AppHandle,
        app: &str,
    ) -> Result<AHQStoreApplication> {
        Ok(internet::get_app(&appl.ahqstore().commits.read().await.commit, app).await?)
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__get_app;
    #[allow(
        unreachable_code,
        clippy::diverging_sub_expression,
        clippy::used_underscore_binding
    )]
    const _: () = if false {
        #[diagnostic::on_unimplemented(
            message = "async commands that contain references as inputs must return a `Result`"
        )]
        trait AsyncCommandMustReturnResult {}
        impl<A, B> AsyncCommandMustReturnResult for ::std::result::Result<A, B> {}
        let _check: Result<Response> = ::core::panicking::panic(
            "internal error: entered unreachable code",
        );
        let _: &dyn AsyncCommandMustReturnResult = &_check;
    };
    pub(crate) async fn get_app_asset(
        appl: AppHandle,
        app: &str,
        asset: &str,
    ) -> Result<Response> {
        let bytes = internet::get_app_asset(
                &appl.ahqstore().commits.read().await.commit,
                app,
                asset,
            )
            .await
            .context("")?;
        Ok(Response::new(bytes))
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__get_app_asset;
    #[allow(
        unreachable_code,
        clippy::diverging_sub_expression,
        clippy::used_underscore_binding
    )]
    const _: () = if false {
        #[diagnostic::on_unimplemented(
            message = "async commands that contain references as inputs must return a `Result`"
        )]
        trait AsyncCommandMustReturnResult {}
        impl<A, B> AsyncCommandMustReturnResult for ::std::result::Result<A, B> {}
        let _check: Result<DevData> = ::core::panicking::panic(
            "internal error: entered unreachable code",
        );
        let _: &dyn AsyncCommandMustReturnResult = &_check;
    };
    pub(crate) async fn get_dev_data(app: AppHandle, dev: &str) -> Result<DevData> {
        Ok(
            internet::get_dev_data(&app.ahqstore().commits.read().await.commit, dev)
                .await?,
        )
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__get_dev_data;
    #[allow(
        unreachable_code,
        clippy::diverging_sub_expression,
        clippy::used_underscore_binding
    )]
    const _: () = if false {
        #[diagnostic::on_unimplemented(
            message = "async commands that contain references as inputs must return a `Result`"
        )]
        trait AsyncCommandMustReturnResult {}
        impl<A, B> AsyncCommandMustReturnResult for ::std::result::Result<A, B> {}
        let _check: Result<Vec<String>> = ::core::panicking::panic(
            "internal error: entered unreachable code",
        );
        let _: &dyn AsyncCommandMustReturnResult = &_check;
    };
    pub(crate) async fn get_devs_apps(app: AppHandle, dev: &str) -> Result<Vec<String>> {
        Ok(
            internet::get_devs_apps(&app.ahqstore().commits.read().await.commit, dev)
                .await?,
        )
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__get_devs_apps;
    pub(crate) fn hash_username(username: String) -> String {
        ahqstore_gh_hash::compute(username.as_str())
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__hash_username;
    pub(crate) fn show_code<R: Runtime>(app: AppHandle<R>, code: String) {
        WebviewWindowBuilder::new(
                &app,
                "code",
                tauri::WebviewUrl::App(
                    PathBuf::from(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("/{0}", code))
                        }),
                    ),
                ),
            )
            .skip_taskbar(true)
            .title("Login to GitHub")
            .inner_size(400.0, 150.0)
            .max_inner_size(400.0, 150.0)
            .min_inner_size(400.0, 150.0)
            .decorations(false)
            .always_on_top(true)
            .fullscreen(false)
            .content_protected(true)
            .maximizable(false)
            .minimizable(false)
            .closable(true)
            .focused(true)
            .build();
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__show_code;
    pub(crate) fn rem_code<R: Runtime>(app: tauri::AppHandle<R>) {
        app.get_webview_window("code").unwrap().destroy().unwrap()
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__rem_code;
    pub(crate) fn is_development() -> bool {
        true || "1.0.0".contains("-alpha")
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__is_development;
    pub(crate) fn open(url: String) -> Option<()> {
        match open_2::that(url) {
            Ok(_) => Some(()),
            _ => None,
        }
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__open;
    pub(crate) async fn now() -> u64 {
        use std::time::UNIX_EPOCH;
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
    pub(crate) fn set_progress(
        window: tauri::WebviewWindow<tauri::Wry>,
        state: i32,
        c: Option<u64>,
        t: Option<u64>,
    ) {
        let progress = match (c, t) {
            (Some(c), Some(t)) => Some((c * 100) / t),
            _ => None,
        };
        let _ = window
            .set_progress_bar(ProgressBarState {
                progress,
                status: Some(
                    match state {
                        1 => ProgressBarStatus::Indeterminate,
                        2 => ProgressBarStatus::Normal,
                        4 => ProgressBarStatus::Error,
                        8 => ProgressBarStatus::Paused,
                        _ => ProgressBarStatus::None,
                    },
                ),
            });
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__set_progress;
    pub(crate) fn get_linux_distro() -> Option<String> {
        return None;
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__get_linux_distro;
    pub(crate) fn get_windows() -> &'static str {
        return { if is_windows_11() { "11" } else { "10" } };
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__get_windows;
    pub(crate) fn get_arch() -> &'static str {
        std::env::consts::ARCH
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__get_arch;
    pub(crate) fn is_windows_11() -> bool {
        use std::os::windows::process::CommandExt;
        use std::process::{Command, Stdio};
        let version = Command::new("cmd")
            .creation_flags(0x08000000)
            .args(["/c", "ver"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap()
            .stdout;
        let string = String::from_utf8(version).unwrap();
        let splitted = string
            .replace("\n", "")
            .replace("Microsoft Windows [", "")
            .replace("]", "");
        let version: Vec<&str> = splitted.split(".").collect();
        let version: usize = version[2].parse().unwrap_or(0);
        version >= 22000
    }
    #[allow(unused_imports)]
    pub(crate) use __cmd__is_windows_11;
}
use commands::*;
use crate::structs::platform::*;
mod error {
    use std::string::FromUtf8Error;
    use serde::{ser::Serializer, Serialize};
    pub type Result<T> = std::result::Result<T, Error>;
    pub enum Error {
        #[error("The platform is not supported")]
        UnsupportedPlatform,
        #[error("There was an error while performing search")]
        SearchError,
        #[error("You cannot safely update commits")]
        CannotUpdate,
        #[error(transparent)]
        Tauri(#[from] tauri::Error),
        #[error(transparent)]
        Io(#[from] std::io::Error),
        #[error(transparent)]
        String(#[from] FromUtf8Error),
        #[error(transparent)]
        AHQStore(#[from] anyhow::Error),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Error::UnsupportedPlatform => {
                    ::core::fmt::Formatter::write_str(f, "UnsupportedPlatform")
                }
                Error::SearchError => ::core::fmt::Formatter::write_str(f, "SearchError"),
                Error::CannotUpdate => {
                    ::core::fmt::Formatter::write_str(f, "CannotUpdate")
                }
                Error::Tauri(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Tauri",
                        &__self_0,
                    )
                }
                Error::Io(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Io", &__self_0)
                }
                Error::String(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "String",
                        &__self_0,
                    )
                }
                Error::AHQStore(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AHQStore",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::thiserror::__private17::Error for Error {
        fn source(
            &self,
        ) -> ::core::option::Option<&(dyn ::thiserror::__private17::Error + 'static)> {
            use ::thiserror::__private17::AsDynError as _;
            #[allow(deprecated)]
            match self {
                Error::UnsupportedPlatform { .. } => ::core::option::Option::None,
                Error::SearchError { .. } => ::core::option::Option::None,
                Error::CannotUpdate { .. } => ::core::option::Option::None,
                Error::Tauri { 0: transparent } => {
                    ::thiserror::__private17::Error::source(transparent.as_dyn_error())
                }
                Error::Io { 0: transparent } => {
                    ::thiserror::__private17::Error::source(transparent.as_dyn_error())
                }
                Error::String { 0: transparent } => {
                    ::thiserror::__private17::Error::source(transparent.as_dyn_error())
                }
                Error::AHQStore { 0: transparent } => {
                    ::thiserror::__private17::Error::source(transparent.as_dyn_error())
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::core::fmt::Display for Error {
        fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                Error::UnsupportedPlatform {} => {
                    __formatter.write_str("The platform is not supported")
                }
                Error::SearchError {} => {
                    __formatter.write_str("There was an error while performing search")
                }
                Error::CannotUpdate {} => {
                    __formatter.write_str("You cannot safely update commits")
                }
                Error::Tauri(_0) => ::core::fmt::Display::fmt(_0, __formatter),
                Error::Io(_0) => ::core::fmt::Display::fmt(_0, __formatter),
                Error::String(_0) => ::core::fmt::Display::fmt(_0, __formatter),
                Error::AHQStore(_0) => ::core::fmt::Display::fmt(_0, __formatter),
            }
        }
    }
    #[allow(
        deprecated,
        unused_qualifications,
        clippy::elidable_lifetime_names,
        clippy::needless_lifetimes,
    )]
    #[automatically_derived]
    impl ::core::convert::From<tauri::Error> for Error {
        fn from(source: tauri::Error) -> Self {
            Error::Tauri { 0: source }
        }
    }
    #[allow(
        deprecated,
        unused_qualifications,
        clippy::elidable_lifetime_names,
        clippy::needless_lifetimes,
    )]
    #[automatically_derived]
    impl ::core::convert::From<std::io::Error> for Error {
        fn from(source: std::io::Error) -> Self {
            Error::Io { 0: source }
        }
    }
    #[allow(
        deprecated,
        unused_qualifications,
        clippy::elidable_lifetime_names,
        clippy::needless_lifetimes,
    )]
    #[automatically_derived]
    impl ::core::convert::From<FromUtf8Error> for Error {
        fn from(source: FromUtf8Error) -> Self {
            Error::String { 0: source }
        }
    }
    #[allow(
        deprecated,
        unused_qualifications,
        clippy::elidable_lifetime_names,
        clippy::needless_lifetimes,
    )]
    #[automatically_derived]
    impl ::core::convert::From<anyhow::Error> for Error {
        fn from(source: anyhow::Error) -> Self {
            Error::AHQStore { 0: source }
        }
    }
    impl Serialize for Error {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(self.to_string().as_ref())
        }
    }
}
mod models {
    use serde::{Deserialize, Serialize};
    #[serde(rename_all = "camelCase")]
    pub struct AndroidBuildOutput {
        pub sdk: u32,
        pub release: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AndroidBuildOutput {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "AndroidBuildOutput",
                "sdk",
                &self.sdk,
                "release",
                &&self.release,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for AndroidBuildOutput {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "sdk" => _serde::__private228::Ok(__Field::__field0),
                            "release" => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"sdk" => _serde::__private228::Ok(__Field::__field0),
                            b"release" => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<AndroidBuildOutput>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AndroidBuildOutput;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct AndroidBuildOutput",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct AndroidBuildOutput with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct AndroidBuildOutput with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(AndroidBuildOutput {
                            sdk: __field0,
                            release: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<u32> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<String> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("sdk"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "release",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("sdk")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("release")?
                            }
                        };
                        _serde::__private228::Ok(AndroidBuildOutput {
                            sdk: __field0,
                            release: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["sdk", "release"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "AndroidBuildOutput",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<AndroidBuildOutput>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for AndroidBuildOutput {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "AndroidBuildOutput",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "sdk",
                    &self.sdk,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "release",
                    &self.release,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[serde(rename_all = "camelCase")]
    pub struct ShowCodeRequest {
        pub value: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ShowCodeRequest {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ShowCodeRequest",
                "value",
                &&self.value,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ShowCodeRequest {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "value" => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"value" => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<ShowCodeRequest>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ShowCodeRequest;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct ShowCodeRequest",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ShowCodeRequest with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(ShowCodeRequest { value: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("value")?
                            }
                        };
                        _serde::__private228::Ok(ShowCodeRequest { value: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["value"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ShowCodeRequest",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<ShowCodeRequest>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ShowCodeRequest {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ShowCodeRequest",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "value",
                    &self.value,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[serde(rename_all = "camelCase")]
    pub struct ZoomRequest {
        pub zoom: f32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ZoomRequest {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ZoomRequest",
                "zoom",
                &&self.zoom,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ZoomRequest {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "zoom" => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"zoom" => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<ZoomRequest>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ZoomRequest;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct ZoomRequest",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ZoomRequest with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(ZoomRequest { zoom: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<f32> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("zoom"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("zoom")?
                            }
                        };
                        _serde::__private228::Ok(ZoomRequest { zoom: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["zoom"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ZoomRequest",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<ZoomRequest>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ZoomRequest {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ZoomRequest",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "zoom",
                    &self.zoom,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[serde(rename_all = "camelCase", tag = "event", content = "data")]
    pub enum DownloadEvent {
        #[serde(rename_all = "camelCase")]
        Started { length: u64 },
        #[serde(rename_all = "camelCase")]
        Progress { progress: f64 },
        #[serde(rename_all = "camelCase")]
        Finished {},
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DownloadEvent {
        #[inline]
        fn clone(&self) -> DownloadEvent {
            match self {
                DownloadEvent::Started { length: __self_0 } => {
                    DownloadEvent::Started {
                        length: ::core::clone::Clone::clone(__self_0),
                    }
                }
                DownloadEvent::Progress { progress: __self_0 } => {
                    DownloadEvent::Progress {
                        progress: ::core::clone::Clone::clone(__self_0),
                    }
                }
                DownloadEvent::Finished {} => DownloadEvent::Finished {},
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for DownloadEvent {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    DownloadEvent::Started { ref length } => {
                        #[doc(hidden)]
                        struct __AdjacentlyTagged<'__a> {
                            data: (&'__a u64,),
                            phantom: _serde::__private228::PhantomData<DownloadEvent>,
                        }
                        #[automatically_derived]
                        impl<'__a> _serde::Serialize for __AdjacentlyTagged<'__a> {
                            fn serialize<__S>(
                                &self,
                                __serializer: __S,
                            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                #[allow(unused_variables)]
                                let (length,) = self.data;
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "started",
                                    0 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "length",
                                    length,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                        }
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "DownloadEvent",
                            2,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "event",
                            &_serde::__private228::ser::AdjacentlyTaggedEnumVariant {
                                enum_name: "DownloadEvent",
                                variant_index: 0u32,
                                variant_name: "started",
                            },
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "data",
                            &__AdjacentlyTagged {
                                data: (length,),
                                phantom: _serde::__private228::PhantomData::<DownloadEvent>,
                            },
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    DownloadEvent::Progress { ref progress } => {
                        #[doc(hidden)]
                        struct __AdjacentlyTagged<'__a> {
                            data: (&'__a f64,),
                            phantom: _serde::__private228::PhantomData<DownloadEvent>,
                        }
                        #[automatically_derived]
                        impl<'__a> _serde::Serialize for __AdjacentlyTagged<'__a> {
                            fn serialize<__S>(
                                &self,
                                __serializer: __S,
                            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                #[allow(unused_variables)]
                                let (progress,) = self.data;
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "progress",
                                    0 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "progress",
                                    progress,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                        }
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "DownloadEvent",
                            2,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "event",
                            &_serde::__private228::ser::AdjacentlyTaggedEnumVariant {
                                enum_name: "DownloadEvent",
                                variant_index: 1u32,
                                variant_name: "progress",
                            },
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "data",
                            &__AdjacentlyTagged {
                                data: (progress,),
                                phantom: _serde::__private228::PhantomData::<DownloadEvent>,
                            },
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    DownloadEvent::Finished {} => {
                        #[doc(hidden)]
                        struct __AdjacentlyTagged {
                            data: (),
                            phantom: _serde::__private228::PhantomData<DownloadEvent>,
                        }
                        #[automatically_derived]
                        impl _serde::Serialize for __AdjacentlyTagged {
                            fn serialize<__S>(
                                &self,
                                __serializer: __S,
                            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                #[allow(unused_variables)]
                                let () = self.data;
                                let __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "finished",
                                    0,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                        }
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "DownloadEvent",
                            2,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "event",
                            &_serde::__private228::ser::AdjacentlyTaggedEnumVariant {
                                enum_name: "DownloadEvent",
                                variant_index: 2u32,
                                variant_name: "finished",
                            },
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "data",
                            &__AdjacentlyTagged {
                                data: (),
                                phantom: _serde::__private228::PhantomData::<DownloadEvent>,
                            },
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                }
            }
        }
    };
    #[serde(rename_all = "camelCase", tag = "event", content = "data")]
    pub enum AppInstallStatus {
        Downloading(DownloadEvent),
        AVScanning,
        Installing,
        AppInstallStat(InstallStat),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AppInstallStatus {
        #[inline]
        fn clone(&self) -> AppInstallStatus {
            match self {
                AppInstallStatus::Downloading(__self_0) => {
                    AppInstallStatus::Downloading(::core::clone::Clone::clone(__self_0))
                }
                AppInstallStatus::AVScanning => AppInstallStatus::AVScanning,
                AppInstallStatus::Installing => AppInstallStatus::Installing,
                AppInstallStatus::AppInstallStat(__self_0) => {
                    AppInstallStatus::AppInstallStat(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for AppInstallStatus {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    AppInstallStatus::Downloading(ref __field0) => {
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "AppInstallStatus",
                            2,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "event",
                            &_serde::__private228::ser::AdjacentlyTaggedEnumVariant {
                                enum_name: "AppInstallStatus",
                                variant_index: 0u32,
                                variant_name: "downloading",
                            },
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "data",
                            __field0,
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    AppInstallStatus::AVScanning => {
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "AppInstallStatus",
                            1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "event",
                            &_serde::__private228::ser::AdjacentlyTaggedEnumVariant {
                                enum_name: "AppInstallStatus",
                                variant_index: 1u32,
                                variant_name: "aVScanning",
                            },
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    AppInstallStatus::Installing => {
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "AppInstallStatus",
                            1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "event",
                            &_serde::__private228::ser::AdjacentlyTaggedEnumVariant {
                                enum_name: "AppInstallStatus",
                                variant_index: 2u32,
                                variant_name: "installing",
                            },
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    AppInstallStatus::AppInstallStat(ref __field0) => {
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "AppInstallStatus",
                            2,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "event",
                            &_serde::__private228::ser::AdjacentlyTaggedEnumVariant {
                                enum_name: "AppInstallStatus",
                                variant_index: 3u32,
                                variant_name: "appInstallStat",
                            },
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "data",
                            __field0,
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                }
            }
        }
    };
    #[serde(rename_all = "camelCase", tag = "event", content = "data")]
    pub enum InstallStat {
        AVFailed,
        Installed,
        InstallFailed,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InstallStat {
        #[inline]
        fn clone(&self) -> InstallStat {
            match self {
                InstallStat::AVFailed => InstallStat::AVFailed,
                InstallStat::Installed => InstallStat::Installed,
                InstallStat::InstallFailed => InstallStat::InstallFailed,
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for InstallStat {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    InstallStat::AVFailed => {
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "InstallStat",
                            1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "event",
                            &_serde::__private228::ser::AdjacentlyTaggedEnumVariant {
                                enum_name: "InstallStat",
                                variant_index: 0u32,
                                variant_name: "aVFailed",
                            },
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    InstallStat::Installed => {
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "InstallStat",
                            1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "event",
                            &_serde::__private228::ser::AdjacentlyTaggedEnumVariant {
                                enum_name: "InstallStat",
                                variant_index: 1u32,
                                variant_name: "installed",
                            },
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                    InstallStat::InstallFailed => {
                        let mut __struct = _serde::Serializer::serialize_struct(
                            __serializer,
                            "InstallStat",
                            1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __struct,
                            "event",
                            &_serde::__private228::ser::AdjacentlyTaggedEnumVariant {
                                enum_name: "InstallStat",
                                variant_index: 2u32,
                                variant_name: "installFailed",
                            },
                        )?;
                        _serde::ser::SerializeStruct::end(__struct)
                    }
                }
            }
        }
    };
}
pub use error::{Error, Result};
use structs::Ahqstore;
/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the ahqstore APIs.
pub trait AhqstoreExt<R: Runtime> {
    fn ahqstore(&self) -> &Ahqstore<R>;
}
impl<R: Runtime, T: Manager<R>> crate::AhqstoreExt<R> for T {
    fn ahqstore(&self) -> &Ahqstore<R> {
        self.state::<Ahqstore<R>>().inner()
    }
}
/// Initializes the plugin.
pub fn init() -> TauriPlugin<tauri::Wry> {
    Builder::new("ahqstore")
        .invoke_handler(move |__tauri_invoke__| {
            let __tauri_cmd__ = __tauri_invoke__.message.command();
            match __tauri_cmd__ {
                "get_windows" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = get_windows();
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "get_linux_distro" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = get_linux_distro();
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "is_windows_11" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = is_windows_11();
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "download" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = download(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "download",
                                            key: "handle",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "download",
                                            key: "appId",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "download",
                                            key: "prog",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "encrypt" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = encrypt(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "encrypt",
                                            key: "payload",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "decrypt" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = decrypt(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "decrypt",
                                            key: "encrypted",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "open" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = open(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "open",
                                            key: "url",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "set_progress" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = set_progress(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "set_progress",
                                            key: "window",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "set_progress",
                                            key: "state",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "set_progress",
                                            key: "c",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "set_progress",
                                            key: "t",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "is_development" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = is_development();
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "show_code" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = show_code(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "show_code",
                                            key: "app",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "show_code",
                                            key: "code",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "rem_code" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = rem_code(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "rem_code",
                                            key: "app",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "hash_username" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = hash_username(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "hash_username",
                                            key: "username",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "set_scale" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = set_scale(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "set_scale",
                                            key: "window",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "set_scale",
                                            key: "scale",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "refresh_commit" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = refresh_commit(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "refresh_commit",
                                            key: "app",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "get_commit" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = get_commit(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_commit",
                                            key: "app",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "get_all_search" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = get_all_search(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_all_search",
                                            key: "app",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_all_search",
                                            key: "query",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "get_home" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = get_home(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_home",
                                            key: "app",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "get_app" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = get_app(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_app",
                                            key: "appl",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_app",
                                            key: "app",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "get_dev_data" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = get_dev_data(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_dev_data",
                                            key: "app",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_dev_data",
                                            key: "dev",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "get_app_asset" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = get_app_asset(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_app_asset",
                                            key: "appl",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_app_asset",
                                            key: "app",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_app_asset",
                                            key: "asset",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "get_devs_apps" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = get_devs_apps(
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_devs_apps",
                                            key: "app",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                        ::tauri::ipc::CommandArg::from_command(::tauri::ipc::CommandItem {
                                            plugin: ::core::option::Option::Some("ahqstore"),
                                            name: "get_devs_apps",
                                            key: "dev",
                                            message: &__tauri_message__,
                                            acl: &__tauri_acl__,
                                        })?,
                                    );
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                "get_arch" => {
                    ({
                        move || {
                            #[allow(unused_imports)]
                            use ::tauri::ipc::private::*;
                            #[allow(unused_variables)]
                            let ::tauri::ipc::Invoke {
                                message: __tauri_message__,
                                resolver: __tauri_resolver__,
                                acl: __tauri_acl__,
                            } = __tauri_invoke__;
                            __tauri_resolver__
                                .respond_async_serialized(async move {
                                    let result = get_arch();
                                    let kind = (&result).async_kind();
                                    kind.future(result).await
                                });
                            return true;
                        }
                    })()
                }
                _ => {
                    return false;
                }
            }
        })
        .setup(|app, api| {
            let ahqstore = structs::init(app, api)?;
            app.manage(ahqstore);
            app.ahqstore().init(app.clone());
            Ok(())
        })
        .build()
}
