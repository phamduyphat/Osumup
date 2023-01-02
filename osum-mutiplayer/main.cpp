#include "libs.h"
using json = nlohmann::json;

std::vector<std::string> split (std::string s, std::string delimiter);
bool GetDefaultBrowserLaunchPath(LPTSTR *pszFilepath);
std::string osu_beatmap_link_extract_id(Url &url);
void print_args(int32_t argc, char *argv[]){for(int32_t i = 0; i < argc; i ++) std::cout << argv[i] << ' '; std::cout << std::endl;}


int32_t main(int32_t argc, char *argv[])
{
    std::string url1 = argv[1]; Url url = url1;
    print_args(argc, argv);//debug

    if (std::regex_match(url.str(), std::regex("^(http|https):\/\/osu.ppy.sh\/(b|beatmapsets|beatmaps)\/(.*)")))
    {
        std::string osum = "osum-direct-web:?" + osu_beatmap_link_extract_id(url);

        std::cout << osum << std::endl << url.str() << std::endl;
        //verbose

        ShellExecuteA(NULL, "open", osum.c_str(), NULL, NULL, SW_SHOWDEFAULT);
    }
    else
    {
        url.run("C://Users//phamd//AppData//Local//Programs//Opera GX//Launcher.exe");
        /*
        LPTSTR PATH;
        if (GetDefaultBrowserLaunchPath(&PATH))
        {
            std::cout << PATH << std::endl;
            url.run(PATH);
            delete [] PATH;
        
        }   
        */   
    }
    return 0;
}

bool GetDefaultBrowserLaunchPath(LPTSTR *pszFilepath)
{
    bool    bRes            = false;
    *pszFilepath            = 0;
    HKEY    hKey            = 0;
    TCHAR    szData[1024]    = {0};
    DWORD    dwDataSize        = 0;
    //
    // Vista+ case
    //
    if (ERROR_SUCCESS == RegOpenKeyEx(HKEY_CURRENT_USER, TEXT(
         "Software\\Microsoft\\Windows\\Shell\\Associations\\UrlAssociations\\ftp\\UserChoice"),
        0, KEY_QUERY_VALUE, &hKey))
    {
        //
        // Vista+ does not always have the registry entry we use in the WinXP case (?)
        // So we do a workaround:
        //        1. Read the current browser Progid value from HKCU (current user!)
        //        2. Use this Progid to get the browser command line from global HKCR
        ///          (as every browser in the system writes its command line into HKCR)
        //
        dwDataSize    = ARRAYSIZE(szData)*sizeof(szData[0]);
        if (ERROR_SUCCESS != RegQueryValueEx
        (hKey, TEXT("Progid"), 0, 0, (LPBYTE)&szData, &dwDataSize))
            goto Cleanup;
        if (!dwDataSize)
            goto Cleanup;
        RegCloseKey(hKey); hKey = 0;
        _tcscat_s(szData, ARRAYSIZE(szData), TEXT("\\shell\\open\\command"));
        if (ERROR_SUCCESS != RegOpenKeyEx(HKEY_CLASSES_ROOT, szData, 
            0, KEY_QUERY_VALUE, &hKey))                                   // Using HKCR (!)
            goto Cleanup;
    }
    else
    {
        //
        // WinXP case
        //
        if (ERROR_SUCCESS != RegOpenKeyEx(HKEY_CURRENT_USER, TEXT(
                 "Software\\Classes\\http\\shell\\open\\command"), 
            0, KEY_QUERY_VALUE, &hKey))        // Using HKCU in WinXP (!)
            goto Cleanup;
    }

    if (ERROR_SUCCESS != RegQueryValueEx(hKey, 0, 0, 0, 0, &dwDataSize))
    // Get size in bytes of the default key's value
        goto Cleanup;

    DWORD nMaxSize; nMaxSize        = dwDataSize/sizeof(TCHAR)    // Buf size in chars
        + 3            // +3 chars to reserve the space for an optional " %1" param
        + 1;        // +1 char for \0 terminator
    *pszFilepath    = new TCHAR[nMaxSize];
    if (ERROR_SUCCESS != RegQueryValueEx(hKey, 0, 0, 0, (LPBYTE)(*pszFilepath), &dwDataSize))
    {
        delete [] (*pszFilepath);
        *pszFilepath = 0;
        goto Cleanup;
    }
    if (!_tcsstr(*pszFilepath, TEXT("%1")))
        _tcscat_s(*pszFilepath, nMaxSize, TEXT(" %1"));
        // Add a URL placeholder (it is missing in IE) to the end of the command line
    bRes = true;
Cleanup:
    if (hKey)
        RegCloseKey(hKey);
    return bRes;
}   
std::string osu_beatmap_link_extract_id(Url &url)
{
    std::vector<std::string> uri = split(url.path(),"/");
    return uri[2];
}
std::vector<std::string> split (std::string s, std::string delimiter) {
    size_t pos_start = 0, pos_end, delim_len = delimiter.length();
    std::string token;
    std::vector<std::string> res;

    while ((pos_end = s.find (delimiter, pos_start)) != std::string::npos) {
        token = s.substr (pos_start, pos_end - pos_start);
        pos_start = pos_end + delim_len;
        res.push_back (token);
    }

    res.push_back (s.substr (pos_start));
    return res;
}
// Todo:
//// -somehow has this connect as the default browser                    
//// -receive osu-multi links                                            
//// -convert the link to something that osum-direct-web can use         
//// -run osum-direct-web with the args
// -convert osu-non beatmaps link + non-osu links to brave browser //half - done
// -block certain website(adaf youtube channel, etc...);


