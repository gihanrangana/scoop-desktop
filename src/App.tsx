import { useLayoutEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import styles from './App.module.scss';
import PackageList from './components/PackageList/PackageList';

type ScoopStatus = {
    installed: boolean;
    version?: string;
    path?: string;
    update_available?: boolean;
};

function App() {
    const [scoopStatus, setScoopStatus] = useState<ScoopStatus>();

    useLayoutEffect(() => {
        (async () => {
            const status = await invoke('check_scoop_installed');
            setScoopStatus(status as ScoopStatus);
        })();
    }, []);

    return <div className={styles.container}>
        {/* <div className={styles.content}> */}
            {scoopStatus?.installed && <PackageList />}
        {/* </div> */}
    </div>;
}

export default App;
