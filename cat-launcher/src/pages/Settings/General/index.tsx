import { FC, useEffect } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  FormField,
  FormItem,
  FormLabel,
  FormControl,
  FormDescription,
  FormMessage,
  Form,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { getSettings, Settings } from "@/store/settingsSlice";

const generalSettingsSchema = z.object({
  max_backups: z.coerce.number().min(0).max(20),
  parallel_requests: z.coerce.number().min(1).max(16),
});

export const GeneralSettings: FC = () => {
  const dispatch = useAppDispatch();
  const { settings, loading } = useAppSelector((state) => state.settings);

  const form = useForm<z.infer<typeof generalSettingsSchema>>({
    resolver: zodResolver(generalSettingsSchema),
    defaultValues: settings,
  });

  useEffect(() => {
    dispatch(getSettings());
  }, [dispatch]);

  useEffect(() => {
    if (!loading) {
      form.reset(settings);
    }
  }, [loading, settings, form]);

  const onSubmit = async (values: z.infer<typeof generalSettingsSchema>) => {
    await invoke("update_settings", { settings: values });
  };

  const onCancel = () => {
    form.reset(settings);
  };

  const onReset = () => {
    form.reset({
      max_backups: 5,
      parallel_requests: 4,
    });
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>General</CardTitle>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
            <FormField
              control={form.control}
              name="max_backups"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Max Automatic Backups</FormLabel>
                  <FormControl>
                    <Input type="number" {...field} />
                  </FormControl>
                  <FormDescription>
                    The maximum number of automatic backups to keep. Set to 0 to
                    disable automatic backups. Reducing this number will delete
                    older backups. In particular, setting it to 0, will delete all
                    automatic backups.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="parallel_requests"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Number of parallel network requests</FormLabel>
                  <FormControl>
                    <Input type="number" {...field} />
                  </FormControl>
                  <FormDescription>
                    The number of parallel network requests to use when
                    downloading files. Recommended value is 4 to 8.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className="flex justify-end space-x-2">
              <Button type="button" variant="outline" onClick={onCancel}>
                Cancel
              </Button>
              <Button type="button" variant="outline" onClick={onReset}>
                Reset to Defaults
              </Button>
              <Button type="submit">Apply</Button>
            </div>
          </form>
        </Form>
      </CardContent>
    </Card>
  );
};
