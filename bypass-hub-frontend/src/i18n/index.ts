import i18n from 'i18next';
import {initReactI18next} from 'react-i18next';
import en from './en.json';
import ru from './ru.json';

i18n.use(initReactI18next).init({
    resources: {en: {translation: en}, ru: {translation: ru}},
    lng: navigator.language.startsWith('ru') ? 'ru' : 'en',
    fallbackLng: 'en',
});

export default i18n;