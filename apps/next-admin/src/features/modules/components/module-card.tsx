'use client';

import { Badge } from '@/components/ui/badge';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card';
import { Switch } from '@/components/ui/switch';
import {
  IconPlugConnected,
  IconPlugConnectedX
} from '@tabler/icons-react';
import { useT } from '@/shared/hooks/use-i18n';
import type { ModuleInfo } from '../api';

interface ModuleCardProps {
  module: ModuleInfo;
  loading: boolean;
  onToggle?: (slug: string, enabled: boolean) => void;
}

export function ModuleCard({ module, loading, onToggle }: ModuleCardProps) {
  const { t } = useT();
  const isCore = module.kind === 'core';

  return (
    <Card
      className={`transition-opacity ${!module.enabled && !isCore ? 'opacity-60' : ''}`}
    >
      <CardHeader className='pb-3'>
        <div className='flex items-start justify-between'>
          <div className='flex items-center gap-2'>
            {module.enabled ? (
              <IconPlugConnected className='text-primary h-5 w-5' />
            ) : (
              <IconPlugConnectedX className='text-muted-foreground h-5 w-5' />
            )}
            <CardTitle className='text-base'>{module.name}</CardTitle>
          </div>
          <div className='flex items-center gap-2'>
            {isCore && (
              <Badge variant='default' className='text-xs'>
                {t('modules.badge.core')}
              </Badge>
            )}
            <Badge variant='outline' className='text-xs'>
              v{module.version}
            </Badge>
          </div>
        </div>
        <CardDescription className='text-sm'>
          {module.description}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className='flex items-center justify-between'>
          <div className='text-muted-foreground text-xs'>
            {module.dependencies.length > 0 && (
              <span>
                {t('modules.depends_on')}: {module.dependencies.join(', ')}
              </span>
            )}
          </div>
          {isCore ? (
            <Badge variant='secondary' className='text-xs'>
              {t('modules.always_on')}
            </Badge>
          ) : (
            <div className='flex items-center gap-2'>
              <span className='text-muted-foreground text-xs'>
                {module.enabled ? t('modules.enabled') : t('modules.disabled')}
              </span>
              <Switch
                checked={module.enabled}
                disabled={loading}
                onCheckedChange={(checked) =>
                  onToggle?.(module.moduleSlug, checked)
                }
              />
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  );
}
