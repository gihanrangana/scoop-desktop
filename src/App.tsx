import { useLayoutEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

function App() {
    useLayoutEffect(() => {
        (async () => {
            const status = await invoke('check_scoop_installed');
            console.log(status);
        })();
    }, []);

    return <div></div>;
}

export default App;
