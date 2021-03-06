#![cfg(windows)]

use std::{
    ffi::OsString, fmt::Display, os::windows::ffi::OsStringExt, path::PathBuf, ptr::null_mut,
};

use winapi::{
    shared::guiddef::GUID,
    shared::winerror::{E_FAIL, E_INVALIDARG, HRESULT, S_OK},
    um::{
        combaseapi::CoTaskMemFree, knownfolders::*, shlobj::SHGetKnownFolderPath,
        shtypes::REFKNOWNFOLDERID, winbase::lstrlenW, winnt::PWSTR,
    },
};

#[derive(Debug)]
pub enum Error {
    Virtual,
    NotFound,
    InvalidArg(std::io::Error),
    Other(u32, std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Error::Virtual => "virtual folders have no path",
            Error::NotFound => "not found",
            Error::InvalidArg(_) => "invalid arg",
            Error::Other(_, _) => "other",
        })
    }
}

impl std::error::Error for Error {}

const NOT_FOUND: HRESULT = 0x80070002u32 as i32;
const CANNOT_FIND_PATH: HRESULT = 0x80070003u32 as i32;

fn raw_known_folder_path(id: REFKNOWNFOLDERID) -> Result<PathBuf, Error> {
    let mut ptr: PWSTR = null_mut();
    let ret = unsafe { SHGetKnownFolderPath(id, 0, null_mut(), &mut ptr) };
    let result = match ret {
        S_OK => {
            let len = unsafe { lstrlenW(ptr) } as usize;
            let path = unsafe { std::slice::from_raw_parts(ptr, len) };
            let os_str: OsString = OsString::from_wide(path);
            Ok(PathBuf::from(os_str))
        }
        E_FAIL => Err(Error::Virtual),
        E_INVALIDARG => Err(Error::InvalidArg(std::io::Error::last_os_error())),
        NOT_FOUND | CANNOT_FIND_PATH => Err(Error::NotFound),
        // E_NOTFOUND => Err(Error::NotFound(std::io::Error::last_os_error())),
        e => Err(Error::Other(e as u32, std::io::Error::last_os_error())),
    };

    // Docs say that even if the function fails, you need to free this.
    unsafe { CoTaskMemFree(ptr as *mut _) };

    result
}

#[inline(always)]
pub fn known_folder_path(id: FolderId) -> Result<PathBuf, Error> {
    raw_known_folder_path(&FOLDER_IDS[id as usize])
}

pub enum FolderId {
    NetworkFolder = 0,
    ComputerFolder,
    InternetFolder,
    ControlPanelFolder,
    PrintersFolder,
    SyncManagerFolder,
    SyncSetupFolder,
    ConflictFolder,
    SyncResultsFolder,
    RecycleBinFolder,
    ConnectionsFolder,
    Fonts,
    Desktop,
    Startup,
    Programs,
    StartMenu,
    Recent,
    SendTo,
    Documents,
    Favorites,
    NetHood,
    PrintHood,
    Templates,
    CommonStartup,
    CommonPrograms,
    CommonStartMenu,
    PublicDesktop,
    ProgramData,
    CommonTemplates,
    PublicDocuments,
    RoamingAppData,
    LocalAppData,
    LocalAppDataLow,
    InternetCache,
    Cookies,
    History,
    System,
    SystemX86,
    Windows,
    Profile,
    Pictures,
    ProgramFilesX86,
    ProgramFilesCommonX86,
    ProgramFilesX64,
    ProgramFilesCommonX64,
    ProgramFiles,
    ProgramFilesCommon,
    UserProgramFiles,
    UserProgramFilesCommon,
    AdminTools,
    CommonAdminTools,
    Music,
    Videos,
    Ringtones,
    PublicPictures,
    PublicMusic,
    PublicVideos,
    PublicRingtones,
    ResourceDir,
    LocalizedResourcesDir,
    CommonOEMLinks,
    CDBurning,
    UserProfiles,
    Playlists,
    SamplePlaylists,
    SampleMusic,
    SamplePictures,
    SampleVideos,
    PhotoAlbums,
    Public,
    ChangeRemovePrograms,
    AppUpdates,
    AddNewPrograms,
    Downloads,
    PublicDownloads,
    SavedSearches,
    QuickLaunch,
    Contacts,
    SidebarParts,
    SidebarDefaultParts,
    PublicGameTasks,
    GameTasks,
    SavedGames,
    Games,
    SearchMapi,
    SearchCsc,
    Links,
    UsersFiles,
    UsersLibraries,
    SearchHome,
    OriginalImages,
    DocumentsLibrary,
    MusicLibrary,
    PicturesLibrary,
    VideosLibrary,
    RecordedTVLibrary,
    HomeGroup,
    HomeGroupCurrentUser,
    DeviceMetadataStore,
    Libraries,
    PublicLibraries,
    UserPinned,
    ImplicitAppShortcuts,
    AccountPictures,
    PublicUserTiles,
    AppsFolder,
    StartMenuAllPrograms,
    CommonStartMenuPlaces,
    ApplicationShortcuts,
    RoamingTiles,
    RoamedTileImages,
    Screenshots,
    CameraRoll,
    SkyDrive,
    OneDrive,
    SkyDriveDocuments,
    SkyDrivePictures,
    SkyDriveMusic,
    SkyDriveCameraRoll,
    SearchHistory,
    SearchTemplates,
    CameraRollLibrary,
    SavedPictures,
    SavedPicturesLibrary,
    RetailDemo,
    Device,
    DevelopmentFiles,
    Objects3D,
    AppCaptures,
    LocalDocuments,
    LocalPictures,
    LocalVideos,
    LocalMusic,
    LocalDownloads,
    RecordedCalls,
    AllAppMods,
    CurrentAppMods,
    AppDataDesktop,
    AppDataDocuments,
    AppDataFavorites,
    AppDataProgramData,
}

static FOLDER_IDS: &[GUID] = &[
    FOLDERID_NetworkFolder,
    FOLDERID_ComputerFolder,
    FOLDERID_InternetFolder,
    FOLDERID_ControlPanelFolder,
    FOLDERID_PrintersFolder,
    FOLDERID_SyncManagerFolder,
    FOLDERID_SyncSetupFolder,
    FOLDERID_ConflictFolder,
    FOLDERID_SyncResultsFolder,
    FOLDERID_RecycleBinFolder,
    FOLDERID_ConnectionsFolder,
    FOLDERID_Fonts,
    FOLDERID_Desktop,
    FOLDERID_Startup,
    FOLDERID_Programs,
    FOLDERID_StartMenu,
    FOLDERID_Recent,
    FOLDERID_SendTo,
    FOLDERID_Documents,
    FOLDERID_Favorites,
    FOLDERID_NetHood,
    FOLDERID_PrintHood,
    FOLDERID_Templates,
    FOLDERID_CommonStartup,
    FOLDERID_CommonPrograms,
    FOLDERID_CommonStartMenu,
    FOLDERID_PublicDesktop,
    FOLDERID_ProgramData,
    FOLDERID_CommonTemplates,
    FOLDERID_PublicDocuments,
    FOLDERID_RoamingAppData,
    FOLDERID_LocalAppData,
    FOLDERID_LocalAppDataLow,
    FOLDERID_InternetCache,
    FOLDERID_Cookies,
    FOLDERID_History,
    FOLDERID_System,
    FOLDERID_SystemX86,
    FOLDERID_Windows,
    FOLDERID_Profile,
    FOLDERID_Pictures,
    FOLDERID_ProgramFilesX86,
    FOLDERID_ProgramFilesCommonX86,
    FOLDERID_ProgramFilesX64,
    FOLDERID_ProgramFilesCommonX64,
    FOLDERID_ProgramFiles,
    FOLDERID_ProgramFilesCommon,
    FOLDERID_UserProgramFiles,
    FOLDERID_UserProgramFilesCommon,
    FOLDERID_AdminTools,
    FOLDERID_CommonAdminTools,
    FOLDERID_Music,
    FOLDERID_Videos,
    FOLDERID_Ringtones,
    FOLDERID_PublicPictures,
    FOLDERID_PublicMusic,
    FOLDERID_PublicVideos,
    FOLDERID_PublicRingtones,
    FOLDERID_ResourceDir,
    FOLDERID_LocalizedResourcesDir,
    FOLDERID_CommonOEMLinks,
    FOLDERID_CDBurning,
    FOLDERID_UserProfiles,
    FOLDERID_Playlists,
    FOLDERID_SamplePlaylists,
    FOLDERID_SampleMusic,
    FOLDERID_SamplePictures,
    FOLDERID_SampleVideos,
    FOLDERID_PhotoAlbums,
    FOLDERID_Public,
    FOLDERID_ChangeRemovePrograms,
    FOLDERID_AppUpdates,
    FOLDERID_AddNewPrograms,
    FOLDERID_Downloads,
    FOLDERID_PublicDownloads,
    FOLDERID_SavedSearches,
    FOLDERID_QuickLaunch,
    FOLDERID_Contacts,
    FOLDERID_SidebarParts,
    FOLDERID_SidebarDefaultParts,
    FOLDERID_PublicGameTasks,
    FOLDERID_GameTasks,
    FOLDERID_SavedGames,
    FOLDERID_Games,
    FOLDERID_SEARCH_MAPI,
    FOLDERID_SEARCH_CSC,
    FOLDERID_Links,
    FOLDERID_UsersFiles,
    FOLDERID_UsersLibraries,
    FOLDERID_SearchHome,
    FOLDERID_OriginalImages,
    FOLDERID_DocumentsLibrary,
    FOLDERID_MusicLibrary,
    FOLDERID_PicturesLibrary,
    FOLDERID_VideosLibrary,
    FOLDERID_RecordedTVLibrary,
    FOLDERID_HomeGroup,
    FOLDERID_HomeGroupCurrentUser,
    FOLDERID_DeviceMetadataStore,
    FOLDERID_Libraries,
    FOLDERID_PublicLibraries,
    FOLDERID_UserPinned,
    FOLDERID_ImplicitAppShortcuts,
    FOLDERID_AccountPictures,
    FOLDERID_PublicUserTiles,
    FOLDERID_AppsFolder,
    FOLDERID_StartMenuAllPrograms,
    FOLDERID_CommonStartMenuPlaces,
    FOLDERID_ApplicationShortcuts,
    FOLDERID_RoamingTiles,
    FOLDERID_RoamedTileImages,
    FOLDERID_Screenshots,
    FOLDERID_CameraRoll,
    FOLDERID_SkyDrive,
    FOLDERID_OneDrive,
    FOLDERID_SkyDriveDocuments,
    FOLDERID_SkyDrivePictures,
    FOLDERID_SkyDriveMusic,
    FOLDERID_SkyDriveCameraRoll,
    FOLDERID_SearchHistory,
    FOLDERID_SearchTemplates,
    FOLDERID_CameraRollLibrary,
    FOLDERID_SavedPictures,
    FOLDERID_SavedPicturesLibrary,
    FOLDERID_RetailDemo,
    FOLDERID_Device,
    FOLDERID_DevelopmentFiles,
    FOLDERID_Objects3D,
    FOLDERID_AppCaptures,
    FOLDERID_LocalDocuments,
    FOLDERID_LocalPictures,
    FOLDERID_LocalVideos,
    FOLDERID_LocalMusic,
    FOLDERID_LocalDownloads,
    FOLDERID_RecordedCalls,
    FOLDERID_AllAppMods,
    FOLDERID_CurrentAppMods,
    FOLDERID_AppDataDesktop,
    FOLDERID_AppDataDocuments,
    FOLDERID_AppDataFavorites,
    FOLDERID_AppDataProgramData,
];

#[cfg(test)]
mod tests {
    #[test]
    fn all_ids() {
        for (i, id) in super::FOLDER_IDS.iter().enumerate() {
            let path = super::raw_known_folder_path(id);
            match path {
                Ok(path) => println!("{}: {}", i, path.display()),
                Err(err) => println!("{}: {}", i, err),
            }
        }
    }
}
