import { PageContainer } from '@/widgets/app-shell';
import { ModulesList } from '@/features/modules/components/modules-list';
import { listModules } from '@/features/modules/api';
import { Suspense } from 'react';

export const metadata = {
  title: 'Dashboard: Modules'
};

async function ModulesContent() {
  const data = await listModules();
  return <ModulesList modules={data.modules} />;
}

export default function Page() {
  return (
    <PageContainer
      scrollable
      pageTitle='Modules'
      pageDescription='Manage platform modules. Core modules are always active and cannot be disabled.'
    >
      <Suspense fallback={<div>Loading modules...</div>}>
        <ModulesContent />
      </Suspense>
    </PageContainer>
  );
}
