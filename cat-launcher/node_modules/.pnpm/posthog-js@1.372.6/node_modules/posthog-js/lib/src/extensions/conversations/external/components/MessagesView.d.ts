import { h } from 'preact';
import { Message } from '../../../../posthog-conversations-types';
import { getStyles } from './styles';
interface MessagesViewProps {
    styles: ReturnType<typeof getStyles>;
    primaryColor: string;
    placeholderText: string;
    messages: Message[];
    inputValue: string;
    isLoading: boolean;
    error: string | null;
    isResolved: boolean;
    onInputChange: (e: Event) => void;
    onKeyDown: (e: KeyboardEvent) => void;
    onSendMessage: () => void;
    onStartNewConversation: () => void;
    messagesEndRef: (el: HTMLDivElement | null) => void;
}
export declare function MessagesView({ styles, primaryColor, placeholderText, messages, inputValue, isLoading, error, isResolved, onInputChange, onKeyDown, onSendMessage, onStartNewConversation, messagesEndRef, }: MessagesViewProps): h.JSX.Element;
export {};
