import React, { useState } from 'react';
import Dashboard from './components/dashboard/Dashboard.jsx';

import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import tabStyles from 'react-tabs/style/react-tabs.css';
import Circuit from './components/circuit/Circuit.jsx';

console.log(tabStyles);

export default function App() {
    const [ index, setIndex ] = useState(0);
    return  <Tabs
                selectedTabClassName={tabStyles.reactTabsTabSelected}
                selectedTabPanelClassName={tabStyles.reactTabsTabPanelSelected}
                disabledTabClassName={tabStyles.reactTabsTabDisabled}
                forceRenderTabPanel={true}
                onSelect={i => {
                    setIndex(i);
                    return true;
                }}
            >
                <TabList className={tabStyles.reactTabsTabList}>
                    <Tab className={tabStyles.reactTabsTab}>Grover's Algorithm</Tab>
                    <Tab className={tabStyles.reactTabsTab}>Custom Circuits</Tab>
                </TabList>

                <TabPanel className={tabStyles.reactTabsTabPanel}>
                    <Dashboard bits={4} />
                </TabPanel>
                <TabPanel className={tabStyles.reactTabsTabPanel}>
                    <Circuit bits={4} visible={index == 1} />
                </TabPanel>
            </Tabs>;
}
