'use client';

import {
  FormFileUpload,
  FormInput,
  FormSelect,
  FormTextarea
} from '@/shared/ui/forms';
import { Badge } from '@/shared/ui/shadcn/badge';
import { Button } from '@/shared/ui/shadcn/button';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle
} from '@/shared/ui/shadcn/card';
import { Form } from '@/shared/ui/shadcn/form';
import { Product } from '@/shared/constants/mock-api';
import { zodResolver } from '@hookform/resolvers/zod';
import { useRouter } from 'next/navigation';
import { useMemo, useState } from 'react';
import { useForm } from 'react-hook-form';
import * as z from 'zod';

const MAX_FILE_SIZE = 5000000;
const ACCEPTED_IMAGE_TYPES = [
  'image/jpeg',
  'image/jpg',
  'image/png',
  'image/webp'
];

type SuggestedAttributes = {
  brand: string;
  material: string;
  color: string;
  size: string;
  dimensions: string;
  compatibility: string;
  careInstructions: string;
  hazmat: string;
};

const DEFAULT_SUGGESTED_ATTRIBUTES: SuggestedAttributes = {
  brand: '',
  material: '',
  color: '',
  size: '',
  dimensions: '',
  compatibility: '',
  careInstructions: '',
  hazmat: 'none'
};

const formSchema = z.object({
  image: z
    .any()
    .refine((files) => files?.length == 1, 'Image is required.')
    .refine(
      (files) => files?.[0]?.size <= MAX_FILE_SIZE,
      `Max file size is 5MB.`
    )
    .refine(
      (files) => ACCEPTED_IMAGE_TYPES.includes(files?.[0]?.type),
      '.jpg, .jpeg, .png and .webp files are accepted.'
    ),
  name: z.string().min(2, {
    message: 'Product name must be at least 2 characters.'
  }),
  category: z.string(),
  price: z.number(),
  description: z.string().min(10, {
    message: 'Description must be at least 10 characters.'
  }),
  brand: z.string().optional(),
  material: z.string().optional(),
  color: z.string().optional(),
  size: z.string().optional(),
  dimensions: z.string().optional(),
  compatibility: z.string().optional(),
  careInstructions: z.string().optional(),
  hazmat: z.string().optional()
});

export default function ProductForm({
  initialData,
  pageTitle
}: {
  initialData: Product | null;
  pageTitle: string;
}) {
  const [suggestedAttributes, setSuggestedAttributes] =
    useState<SuggestedAttributes | null>(null);

  const defaultValues = {
    name: initialData?.name || '',
    category: initialData?.category || '',
    price: initialData?.price || undefined,
    description: initialData?.description || '',
    ...DEFAULT_SUGGESTED_ATTRIBUTES
  };

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: defaultValues
  });

  const router = useRouter();

  function onSubmit(values: z.infer<typeof formSchema>) {
    console.log(values);
    router.push('/dashboard/product');
  }

  const formValues = form.watch();

  const previewDiff = useMemo(() => {
    if (!suggestedAttributes) {
      return [] as Array<{ field: keyof SuggestedAttributes; current: string; suggested: string }>;
    }

    return (Object.keys(suggestedAttributes) as Array<keyof SuggestedAttributes>)
      .map((field) => ({
        field,
        current: String(formValues[field] ?? ''),
        suggested: String(suggestedAttributes[field] ?? '')
      }))
      .filter((entry) => entry.suggested.trim().length > 0 && entry.current !== entry.suggested);
  }, [formValues, suggestedAttributes]);

  function handleAiFill() {
    const { category, description, name } = form.getValues();
    const normalizedCategory = category.toLowerCase();
    const source = `${name} ${description}`.toLowerCase();

    const categoryDefaults: Record<string, Partial<SuggestedAttributes>> = {
      electronics: {
        material: 'plastic/aluminum',
        compatibility: 'universal',
        careInstructions: 'avoid moisture, clean with dry cloth'
      },
      beauty: {
        material: 'cosmetic formula',
        careInstructions: 'store in a cool dry place'
      },
      home: {
        material: 'mixed materials',
        careInstructions: 'clean with soft cloth'
      },
      sports: {
        material: 'synthetic textile',
        careInstructions: 'hand wash only'
      }
    };

    setSuggestedAttributes({
      brand: 'AI Suggested Brand',
      material: source.includes('leather') ? 'leather' : categoryDefaults[normalizedCategory]?.material || 'not specified',
      color: source.includes('black') ? 'black' : source.includes('white') ? 'white' : 'not specified',
      size: source.includes('xl') ? 'XL' : 'one size',
      dimensions: source.includes('cm') ? 'from description' : 'not specified',
      compatibility: categoryDefaults[normalizedCategory]?.compatibility || 'not specified',
      careInstructions: categoryDefaults[normalizedCategory]?.careInstructions || 'see packaging',
      hazmat: source.includes('battery') ? 'battery' : 'none'
    });
  }

  function handleApplySuggestions() {
    if (!suggestedAttributes) return;
    for (const [field, value] of Object.entries(suggestedAttributes)) {
      form.setValue(field as keyof SuggestedAttributes, value, { shouldDirty: true });
    }
  }

  return (
    <Card className='mx-auto w-full'>
      <CardHeader>
        <CardTitle className='text-left text-2xl font-bold'>
          {pageTitle}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <Form
          form={form}
          onSubmit={form.handleSubmit(onSubmit)}
          className='space-y-8'
        >
          <FormFileUpload
            control={form.control}
            name='image'
            label='Product Image'
            description='Upload a product image'
            config={{
              maxSize: 5 * 1024 * 1024,
              maxFiles: 4
            }}
          />

          <div className='grid grid-cols-1 gap-6 md:grid-cols-2'>
            <FormInput control={form.control} name='name' label='Product Name' placeholder='Enter product name' required />

            <FormSelect
              control={form.control}
              name='category'
              label='Category'
              placeholder='Select category'
              required
              options={[
                { label: 'Beauty Products', value: 'beauty' },
                { label: 'Electronics', value: 'electronics' },
                { label: 'Home & Garden', value: 'home' },
                { label: 'Sports & Outdoors', value: 'sports' }
              ]}
            />

            <FormInput control={form.control} name='price' label='Price' placeholder='Enter price' required type='number' min={0} step='0.01' />
          </div>

          <FormTextarea
            control={form.control}
            name='description'
            label='Description'
            placeholder='Enter product description'
            required
            config={{ maxLength: 500, showCharCount: true, rows: 4 }}
          />

          <Card>
            <CardHeader className='pb-2'>
              <CardTitle className='text-lg'>AI Fill Attributes</CardTitle>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='flex flex-wrap gap-2'>
                <Button type='button' variant='outline' onClick={handleAiFill}>AI Fill</Button>
                <Button type='button' onClick={handleApplySuggestions} disabled={!suggestedAttributes}>Apply Suggested Attributes</Button>
              </div>

              {previewDiff.length > 0 && (
                <div className='rounded-md border p-3'>
                  <p className='mb-2 text-sm font-medium'>Preview diff</p>
                  <ul className='space-y-2 text-sm'>
                    {previewDiff.map((entry) => (
                      <li key={entry.field} className='flex flex-wrap items-center gap-2'>
                        <Badge variant='secondary'>{entry.field}</Badge>
                        <span className='text-muted-foreground'>"{entry.current || '—'}" → "{entry.suggested}"</span>
                      </li>
                    ))}
                  </ul>
                </div>
              )}

              <div className='grid grid-cols-1 gap-4 md:grid-cols-2'>
                <FormInput control={form.control} name='brand' label='Brand' placeholder='Brand' />
                <FormInput control={form.control} name='material' label='Material' placeholder='Material' />
                <FormInput control={form.control} name='color' label='Color' placeholder='Color' />
                <FormInput control={form.control} name='size' label='Size' placeholder='Size' />
                <FormInput control={form.control} name='dimensions' label='Dimensions' placeholder='Dimensions' />
                <FormInput control={form.control} name='compatibility' label='Compatibility' placeholder='Compatibility' />
              </div>
              <FormTextarea control={form.control} name='careInstructions' label='Care instructions' placeholder='Care instructions' config={{ rows: 3 }} />
              <FormInput control={form.control} name='hazmat' label='Hazmat' placeholder='Hazmat classification' />
            </CardContent>
          </Card>

          <Button type='submit'>Add Product</Button>
        </Form>
      </CardContent>
    </Card>
  );
}
