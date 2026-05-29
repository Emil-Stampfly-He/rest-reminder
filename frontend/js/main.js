const THEME_STORAGE_KEY = 'rest-reminder-theme';
const LANGUAGE_STORAGE_KEY = 'rest-reminder-language';
const CONFIG_STORAGE_KEY = 'rest-reminder-config';
const DEFAULT_LANGUAGE = 'en';

const translations = {
  en: {
    lang: 'en',
    appTitle: 'Focus monitoring and work statistics',
    languageLabel: 'Language',
    themeToggle: 'Toggle theme',
    featureNavigation: 'Feature navigation',
    tabRest: 'Start monitoring',
    tabCount: 'Work statistics',
    tabPlot: 'Generate chart',
    restTitle: 'Start rest reminders',
    restDescription: 'Detect selected apps, show a rest reminder after the work threshold, and write sessions to the log.',
    logDirectory: 'Log directory',
    logDirectoryPlaceholder: 'Example: ~/Desktop or D:\\',
    reminderInterval: 'Reminder interval (seconds)',
    monitoredApps: 'Monitored apps',
    appsPlaceholder: 'Example: Cursor, Xcode, Code',
    processSearchPlaceholder: 'Search running processes or type an app name',
    refreshProcesses: 'Refresh',
    loadingProcesses: 'Loading processes...',
    noProcessMatches: 'No matching processes. Press Enter to add your typed name.',
    processLoadFailed: 'Could not load process list.',
    appsHint: 'Search the running process list, or type a name and press Enter to add it manually.',
    selectedAppsEmpty: 'No apps selected yet.',
    noAppsSelected: 'Select at least one app to monitor.',
    removeApp: 'Remove',
    startMonitoring: 'Start monitoring',
    stopMonitoring: 'Stop monitoring',
    monitorStatus: 'Monitor status',
    monitorRunning: 'Running',
    monitorStopped: 'Stopped',
    monitorElapsed: 'Elapsed',
    monitoringApps: 'Monitoring apps',
    noneValue: 'None',
    alreadyRunning: 'Monitoring is already running.',
    stoppedMonitoring: 'Monitoring stopped.',
    countTitle: 'Calculate work time',
    countDescription: 'Read focus_log.txt and calculate total work seconds by date range, single day, or precise time range.',
    statisticsMode: 'Statistics mode',
    modeRange: 'Date range',
    modeSingle: 'Single day',
    modePrecise: 'Precise time',
    logFile: 'Log file',
    logFilePlaceholder: 'Example: ~/Desktop/focus_log.txt',
    startDate: 'Start date',
    endDate: 'End date',
    calculateRange: 'Calculate range',
    date: 'Date',
    calculateSingle: 'Calculate day',
    startTime: 'Start time',
    endTime: 'End time',
    calculatePrecise: 'Calculate precise range',
    plotTitle: 'Generate work trend chart',
    plotDescription: 'Generate a work trend image for the selected date range from the log file.',
    plotLocation: 'Chart save location',
    plotLocationPlaceholder: 'Example: ~/Desktop/plot.png',
    browse: 'Browse',
    generateChart: 'Generate chart',
    logPreviewTitle: 'Recent log entries',
    refreshPreview: 'Refresh preview',
    logPreviewEmpty: 'Select a log file to preview recent entries.',
    logPreviewNoEntries: 'No log entries found.',
    plotPreviewTitle: 'Chart preview',
    plotPreviewAlt: 'Work trend chart preview',
    submitting: 'Submitting...',
    openingPicker: 'Opening picker...',
    pathSelected: 'Path selected.',
    pickerCancelled: 'Selection cancelled.',
    requestFailed: 'Request failed',
    countResult: ({ formatted, seconds }) => `Done: ${formatted} (${seconds} seconds)`,
    plotResult: ({ location }) => `Chart generated: ${location || 'selected location'}`,
    restResult: 'Monitoring started. It will keep running in the background.',
    timeUnits: { hour: 'h', minute: 'm', second: 's' },
  },
  'zh-Hans': {
    lang: 'zh-CN',
    appTitle: '专注监控与工作统计',
    languageLabel: '语言',
    themeToggle: '切换主题',
    featureNavigation: '功能导航',
    tabRest: '开始监控',
    tabCount: '统计时长',
    tabPlot: '生成图表',
    restTitle: '开始休息提醒',
    restDescription: '检测指定应用，连续工作达到阈值后弹出休息提醒，并写入日志。',
    logDirectory: '日志目录',
    logDirectoryPlaceholder: '例如 ~/Desktop 或 D:\\',
    reminderInterval: '提醒间隔（秒）',
    monitoredApps: '监控应用',
    appsPlaceholder: '例如 Cursor, Xcode, Code',
    processSearchPlaceholder: '搜索正在运行的进程，或输入应用名称',
    refreshProcesses: '刷新',
    loadingProcesses: '正在加载进程...',
    noProcessMatches: '没有匹配进程。按 Enter 可添加当前输入的名称。',
    processLoadFailed: '无法加载进程列表。',
    appsHint: '搜索正在运行的进程并点击添加；如果应用未启动，也可以输入名称后按 Enter 手动添加。',
    selectedAppsEmpty: '尚未选择应用。',
    noAppsSelected: '请至少选择一个需要监控的应用。',
    removeApp: '移除',
    startMonitoring: '启动监控',
    stopMonitoring: '停止监控',
    monitorStatus: '监控状态',
    monitorRunning: '运行中',
    monitorStopped: '已停止',
    monitorElapsed: '已运行',
    monitoringApps: '监控应用',
    noneValue: '无',
    alreadyRunning: '监控已经在运行。',
    stoppedMonitoring: '监控已停止。',
    countTitle: '统计工作时长',
    countDescription: '读取 focus_log.txt，按日期范围、单日或精确时间段计算累计工作秒数。',
    statisticsMode: '统计方式',
    modeRange: '日期范围',
    modeSingle: '单日',
    modePrecise: '精确时间',
    logFile: '日志文件',
    logFilePlaceholder: '例如 ~/Desktop/focus_log.txt',
    startDate: '开始日期',
    endDate: '结束日期',
    calculateRange: '统计范围',
    date: '日期',
    calculateSingle: '统计单日',
    startTime: '开始时间',
    endTime: '结束时间',
    calculatePrecise: '精确统计',
    plotTitle: '生成工作趋势图',
    plotDescription: '根据日志文件生成指定日期范围内的工作趋势图片。',
    plotLocation: '图片保存位置',
    plotLocationPlaceholder: '例如 ~/Desktop/plot.png',
    browse: '浏览',
    generateChart: '生成图表',
    logPreviewTitle: '最近日志',
    refreshPreview: '刷新预览',
    logPreviewEmpty: '选择日志文件后可预览最近记录。',
    logPreviewNoEntries: '没有找到日志记录。',
    plotPreviewTitle: '图表预览',
    plotPreviewAlt: '工作趋势图预览',
    submitting: '正在提交...',
    openingPicker: '正在打开选择器...',
    pathSelected: '路径已选择。',
    pickerCancelled: '已取消选择。',
    requestFailed: '请求失败',
    countResult: ({ formatted, seconds }) => `统计完成：${formatted}（${seconds} 秒）`,
    plotResult: ({ location }) => `图表已生成：${location || '指定位置'}`,
    restResult: '监控已启动。它会在后台持续运行。',
    timeUnits: { hour: '小时', minute: '分钟', second: '秒' },
  },
  'zh-Hant': {
    lang: 'zh-Hant',
    appTitle: '專注監控與工作統計',
    languageLabel: '語言',
    themeToggle: '切換主題',
    featureNavigation: '功能導覽',
    tabRest: '開始監控',
    tabCount: '統計時長',
    tabPlot: '產生圖表',
    restTitle: '開始休息提醒',
    restDescription: '偵測指定應用程式，連續工作達到門檻後彈出休息提醒，並寫入日誌。',
    logDirectory: '日誌目錄',
    logDirectoryPlaceholder: '例如 ~/Desktop 或 D:\\',
    reminderInterval: '提醒間隔（秒）',
    monitoredApps: '監控應用程式',
    appsPlaceholder: '例如 Cursor, Xcode, Code',
    processSearchPlaceholder: '搜尋正在執行的程序，或輸入應用程式名稱',
    refreshProcesses: '重新整理',
    loadingProcesses: '正在載入程序...',
    noProcessMatches: '沒有符合的程序。按 Enter 可加入目前輸入的名稱。',
    processLoadFailed: '無法載入程序清單。',
    appsHint: '搜尋正在執行的程序並點擊加入；如果應用程式尚未啟動，也可以輸入名稱後按 Enter 手動加入。',
    selectedAppsEmpty: '尚未選擇應用程式。',
    noAppsSelected: '請至少選擇一個需要監控的應用程式。',
    removeApp: '移除',
    startMonitoring: '啟動監控',
    stopMonitoring: '停止監控',
    monitorStatus: '監控狀態',
    monitorRunning: '執行中',
    monitorStopped: '已停止',
    monitorElapsed: '已執行',
    monitoringApps: '監控應用程式',
    noneValue: '無',
    alreadyRunning: '監控已經在執行。',
    stoppedMonitoring: '監控已停止。',
    countTitle: '統計工作時長',
    countDescription: '讀取 focus_log.txt，依日期範圍、單日或精確時間區間計算累計工作秒數。',
    statisticsMode: '統計方式',
    modeRange: '日期範圍',
    modeSingle: '單日',
    modePrecise: '精確時間',
    logFile: '日誌檔案',
    logFilePlaceholder: '例如 ~/Desktop/focus_log.txt',
    startDate: '開始日期',
    endDate: '結束日期',
    calculateRange: '統計範圍',
    date: '日期',
    calculateSingle: '統計單日',
    startTime: '開始時間',
    endTime: '結束時間',
    calculatePrecise: '精確統計',
    plotTitle: '產生工作趨勢圖',
    plotDescription: '根據日誌檔案，產生指定日期範圍內的工作趨勢圖片。',
    plotLocation: '圖片儲存位置',
    plotLocationPlaceholder: '例如 ~/Desktop/plot.png',
    browse: '瀏覽',
    generateChart: '產生圖表',
    logPreviewTitle: '最近日誌',
    refreshPreview: '重新整理預覽',
    logPreviewEmpty: '選擇日誌檔案後可預覽最近記錄。',
    logPreviewNoEntries: '找不到日誌記錄。',
    plotPreviewTitle: '圖表預覽',
    plotPreviewAlt: '工作趨勢圖預覽',
    submitting: '正在提交...',
    openingPicker: '正在開啟選擇器...',
    pathSelected: '路徑已選擇。',
    pickerCancelled: '已取消選擇。',
    requestFailed: '請求失敗',
    countResult: ({ formatted, seconds }) => `統計完成：${formatted}（${seconds} 秒）`,
    plotResult: ({ location }) => `圖表已產生：${location || '指定位置'}`,
    restResult: '監控已啟動。它會在背景持續執行。',
    timeUnits: { hour: '小時', minute: '分鐘', second: '秒' },
  },
  ja: {
    lang: 'ja',
    appTitle: '集中モニタリングと作業統計',
    languageLabel: '言語',
    themeToggle: 'テーマを切り替え',
    featureNavigation: '機能ナビゲーション',
    tabRest: '監視を開始',
    tabCount: '作業統計',
    tabPlot: 'グラフ作成',
    restTitle: '休憩リマインダーを開始',
    restDescription: '指定したアプリを検出し、作業時間がしきい値に達したら休憩を通知してログに記録します。',
    logDirectory: 'ログディレクトリ',
    logDirectoryPlaceholder: '例: ~/Desktop または D:\\',
    reminderInterval: '通知間隔（秒）',
    monitoredApps: '監視するアプリ',
    appsPlaceholder: '例: Cursor, Xcode, Code',
    processSearchPlaceholder: '実行中のプロセスを検索、またはアプリ名を入力',
    refreshProcesses: '更新',
    loadingProcesses: 'プロセスを読み込み中...',
    noProcessMatches: '一致するプロセスがありません。Enter で入力名を追加できます。',
    processLoadFailed: 'プロセス一覧を読み込めませんでした。',
    appsHint: '実行中のプロセスを検索して追加できます。起動していないアプリは名前を入力して Enter で追加できます。',
    selectedAppsEmpty: 'まだアプリが選択されていません。',
    noAppsSelected: '監視するアプリを少なくとも1つ選択してください。',
    removeApp: '削除',
    startMonitoring: '監視を開始',
    stopMonitoring: '監視を停止',
    monitorStatus: '監視状態',
    monitorRunning: '実行中',
    monitorStopped: '停止中',
    monitorElapsed: '経過時間',
    monitoringApps: '監視中のアプリ',
    noneValue: 'なし',
    alreadyRunning: '監視はすでに実行中です。',
    stoppedMonitoring: '監視を停止しました。',
    countTitle: '作業時間を計算',
    countDescription: 'focus_log.txt を読み込み、日付範囲、単日、または正確な時間範囲で合計作業秒数を計算します。',
    statisticsMode: '統計モード',
    modeRange: '日付範囲',
    modeSingle: '単日',
    modePrecise: '正確な時間',
    logFile: 'ログファイル',
    logFilePlaceholder: '例: ~/Desktop/focus_log.txt',
    startDate: '開始日',
    endDate: '終了日',
    calculateRange: '範囲を計算',
    date: '日付',
    calculateSingle: '1日を計算',
    startTime: '開始時刻',
    endTime: '終了時刻',
    calculatePrecise: '正確な範囲を計算',
    plotTitle: '作業トレンドグラフを作成',
    plotDescription: 'ログファイルから、指定した日付範囲の作業トレンド画像を作成します。',
    plotLocation: 'グラフの保存先',
    plotLocationPlaceholder: '例: ~/Desktop/plot.png',
    browse: '参照',
    generateChart: 'グラフを作成',
    logPreviewTitle: '最近のログ',
    refreshPreview: 'プレビューを更新',
    logPreviewEmpty: 'ログファイルを選択すると最近の記録をプレビューできます。',
    logPreviewNoEntries: 'ログ記録が見つかりません。',
    plotPreviewTitle: 'グラフプレビュー',
    plotPreviewAlt: '作業トレンドグラフのプレビュー',
    submitting: '送信中...',
    openingPicker: '選択画面を開いています...',
    pathSelected: 'パスを選択しました。',
    pickerCancelled: '選択をキャンセルしました。',
    requestFailed: 'リクエストに失敗しました',
    countResult: ({ formatted, seconds }) => `完了: ${formatted}（${seconds} 秒）`,
    plotResult: ({ location }) => `グラフを作成しました: ${location || '指定した場所'}`,
    restResult: '監視を開始しました。バックグラウンドで実行されます。',
    timeUnits: { hour: '時間', minute: '分', second: '秒' },
  },
  fr: {
    lang: 'fr',
    appTitle: 'Suivi de concentration et statistiques de travail',
    languageLabel: 'Langue',
    themeToggle: 'Changer le thème',
    featureNavigation: 'Navigation des fonctionnalités',
    tabRest: 'Démarrer le suivi',
    tabCount: 'Statistiques',
    tabPlot: 'Créer un graphique',
    restTitle: 'Démarrer les rappels de pause',
    restDescription: 'Détecte les applications choisies, affiche un rappel de pause après le seuil de travail, puis écrit les sessions dans le journal.',
    logDirectory: 'Dossier du journal',
    logDirectoryPlaceholder: 'Exemple : ~/Desktop ou D:\\',
    reminderInterval: 'Intervalle de rappel (secondes)',
    monitoredApps: 'Applications suivies',
    appsPlaceholder: 'Exemple : Cursor, Xcode, Code',
    processSearchPlaceholder: 'Rechercher les processus actifs ou saisir une application',
    refreshProcesses: 'Actualiser',
    loadingProcesses: 'Chargement des processus...',
    noProcessMatches: 'Aucun processus correspondant. Appuyez sur Entrée pour ajouter le nom saisi.',
    processLoadFailed: 'Impossible de charger la liste des processus.',
    appsHint: 'Recherchez un processus actif et cliquez pour l’ajouter. Vous pouvez aussi saisir un nom puis appuyer sur Entrée.',
    selectedAppsEmpty: 'Aucune application sélectionnée.',
    noAppsSelected: 'Sélectionnez au moins une application à suivre.',
    removeApp: 'Supprimer',
    startMonitoring: 'Démarrer le suivi',
    stopMonitoring: 'Arrêter le suivi',
    monitorStatus: 'État du suivi',
    monitorRunning: 'En cours',
    monitorStopped: 'Arrêté',
    monitorElapsed: 'Écoulé',
    monitoringApps: 'Applications suivies',
    noneValue: 'Aucun',
    alreadyRunning: 'Le suivi est déjà en cours.',
    stoppedMonitoring: 'Le suivi est arrêté.',
    countTitle: 'Calculer le temps de travail',
    countDescription: 'Lit focus_log.txt et calcule le total en secondes par période, journée unique ou plage horaire précise.',
    statisticsMode: 'Mode statistique',
    modeRange: 'Période',
    modeSingle: 'Jour unique',
    modePrecise: 'Heure précise',
    logFile: 'Fichier journal',
    logFilePlaceholder: 'Exemple : ~/Desktop/focus_log.txt',
    startDate: 'Date de début',
    endDate: 'Date de fin',
    calculateRange: 'Calculer la période',
    date: 'Date',
    calculateSingle: 'Calculer le jour',
    startTime: 'Heure de début',
    endTime: 'Heure de fin',
    calculatePrecise: 'Calculer la plage précise',
    plotTitle: 'Créer le graphique de tendance',
    plotDescription: 'Crée une image de tendance du temps de travail pour la période choisie à partir du journal.',
    plotLocation: 'Emplacement du graphique',
    plotLocationPlaceholder: 'Exemple : ~/Desktop/plot.png',
    browse: 'Parcourir',
    generateChart: 'Créer le graphique',
    logPreviewTitle: 'Entrées récentes',
    refreshPreview: 'Actualiser l’aperçu',
    logPreviewEmpty: 'Sélectionnez un journal pour afficher les entrées récentes.',
    logPreviewNoEntries: 'Aucune entrée trouvée.',
    plotPreviewTitle: 'Aperçu du graphique',
    plotPreviewAlt: 'Aperçu du graphique de tendance',
    submitting: 'Envoi...',
    openingPicker: 'Ouverture du sélecteur...',
    pathSelected: 'Chemin sélectionné.',
    pickerCancelled: 'Sélection annulée.',
    requestFailed: 'La requête a échoué',
    countResult: ({ formatted, seconds }) => `Terminé : ${formatted} (${seconds} secondes)`,
    plotResult: ({ location }) => `Graphique créé : ${location || 'emplacement choisi'}`,
    restResult: 'Le suivi a démarré. Il continue en arrière-plan.',
    timeUnits: { hour: 'h', minute: 'min', second: 's' },
  },
};

let currentLanguage = DEFAULT_LANGUAGE;
let configReady = false;

const t = (key) => translations[currentLanguage]?.[key] ?? translations[DEFAULT_LANGUAGE][key] ?? key;

const formatSeconds = (seconds) => {
  const total = Number(seconds || 0);
  const hours = Math.floor(total / 3600);
  const minutes = Math.floor((total % 3600) / 60);
  const rest = total % 60;
  const units = t('timeUnits');
  return `${hours}${units.hour} ${minutes}${units.minute} ${rest}${units.second}`;
};

const toPreciseApiDate = (value) => value.replace('T', ' ');

const pickerEndpoints = {
  directory: '/dialog/directory',
  file: '/dialog/file',
  'save-file': '/dialog/save-file',
};

const processState = {
  all: [],
  selected: [],
  loaded: false,
  dropdownOpen: false,
};

const preferredProcessTerms = [
  'spotify',
  'rustrover',
  'cursor',
  'code',
  'xcode',
  'intellij',
  'webstorm',
  'pycharm',
  'notion',
  'obsidian',
  'slack',
  'discord',
  'chrome',
  'firefox',
  'safari',
  'terminal',
  'iterm',
  'wechat',
  'qq',
];

const setStatus = (form, message, type = '') => {
  const output = form.querySelector('.status');
  if (!output) return;
  output.textContent = message;
  output.className = `status${type ? ` is-${type}` : ''}`;
};

const translatePage = (language) => {
  currentLanguage = translations[language] ? language : DEFAULT_LANGUAGE;
  const dictionary = translations[currentLanguage];
  document.documentElement.lang = dictionary.lang;

  document.querySelectorAll('[data-i18n]').forEach((element) => {
    element.textContent = t(element.dataset.i18n);
  });

  document.querySelectorAll('[data-i18n-placeholder]').forEach((element) => {
    element.setAttribute('placeholder', t(element.dataset.i18nPlaceholder));
  });

  document.querySelectorAll('[data-i18n-aria-label]').forEach((element) => {
    element.setAttribute('aria-label', t(element.dataset.i18nAriaLabel));
  });

  document.querySelectorAll('[data-i18n-alt]').forEach((element) => {
    element.setAttribute('alt', t(element.dataset.i18nAlt));
  });

  const languageSelect = document.getElementById('language-select');
  if (languageSelect) {
    languageSelect.value = currentLanguage;
  }

  renderSelectedProcesses();
  if (processState.loaded) {
    renderProcessSuggestions();
  }
};

const readForm = (form) => {
  const data = Object.fromEntries(new FormData(form).entries());
  const kind = form.dataset.kind;

  if (kind === 'rest') {
    data.time = Number(data.time);
    data.app_list = processState.selected.length
      ? [...processState.selected]
      : data.app_list
        .split(',')
        .map((item) => item.trim())
        .filter(Boolean);

    if (!data.app_list.length) {
      throw new Error(t('noAppsSelected'));
    }
  }

  if (kind === 'count-precise') {
    data.start_time = toPreciseApiDate(data.start_time);
    data.end_time = toPreciseApiDate(data.end_time);
  }

  return data;
};

function readStoredConfig() {
  try {
    return JSON.parse(localStorage.getItem(CONFIG_STORAGE_KEY) || '{}');
  } catch {
    return {};
  }
}

function saveConfig() {
  const config = {
    rest: {
      logPath: document.querySelector('#rest-panel input[name="log_path"]')?.value || '',
      time: document.querySelector('#rest-panel input[name="time"]')?.value || '3600',
      apps: processState.selected,
    },
    count: {
      logPath: document.querySelector('#count-panel input[name="log_path"]')?.value || '',
    },
    plot: {
      logPath: document.querySelector('#plot-panel input[name="log_path"]')?.value || '',
      plotLocation: document.querySelector('#plot-panel input[name="plot_location"]')?.value || '',
    },
  };

  localStorage.setItem(CONFIG_STORAGE_KEY, JSON.stringify(config));
}

function restoreConfig() {
  const config = readStoredConfig();

  const restLog = document.querySelector('#rest-panel input[name="log_path"]');
  const restTime = document.querySelector('#rest-panel input[name="time"]');
  const countLogs = document.querySelectorAll('#count-panel input[name="log_path"]');
  const plotLog = document.querySelector('#plot-panel input[name="log_path"]');
  const plotLocation = document.querySelector('#plot-panel input[name="plot_location"]');

  if (restLog && config.rest?.logPath) restLog.value = config.rest.logPath;
  if (restTime && config.rest?.time) restTime.value = config.rest.time;
  if (Array.isArray(config.rest?.apps)) {
    processState.selected = [...config.rest.apps];
    renderSelectedProcesses();
  }
  countLogs.forEach((input) => {
    if (config.count?.logPath) input.value = config.count.logPath;
  });
  if (plotLog && config.plot?.logPath) plotLog.value = config.plot.logPath;
  if (plotLocation && config.plot?.plotLocation) plotLocation.value = config.plot.plotLocation;
  configReady = true;
}

const resultMessage = (kind, payload) => {
  if (kind.startsWith('count')) {
    return t('countResult')({
      formatted: formatSeconds(payload.seconds),
      seconds: payload.seconds,
    });
  }

  if (kind === 'plot') {
    return t('plotResult')({ location: payload.plot_location });
  }

  return t('restResult');
};

async function submitForm(form) {
  const button = form.querySelector('button[type="submit"]');
  const endpoint = form.dataset.endpoint;
  const kind = form.dataset.kind;

  setStatus(form, t('submitting'));
  button.disabled = true;

  try {
    const response = await fetch(endpoint, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(readForm(form)),
    });

    const text = await response.text();
    let payload = {};
    try {
      payload = text ? JSON.parse(text) : {};
    } catch {
      payload = { message: text };
    }

    if (!response.ok) {
      throw new Error(payload.error || payload.message || `${t('requestFailed')}: ${response.status}`);
    }

    if (kind === 'plot' && payload.image_data_url) {
      showPlotPreview(payload.image_data_url);
    }

    saveConfig();
    if (kind === 'rest') {
      await refreshMonitorStatus();
    }

    setStatus(form, resultMessage(kind, payload), 'success');
  } catch (error) {
    setStatus(form, error.message || t('requestFailed'), 'error');
  } finally {
    button.disabled = false;
  }
}

function setProcessPickerMessage(message) {
  const container = document.querySelector('[data-process-suggestions]');
  if (!container) return;
  container.innerHTML = '';
  container.classList.toggle('is-open', processState.dropdownOpen);
  const item = document.createElement('div');
  item.className = 'process-message';
  item.textContent = message;
  container.appendChild(item);
}

function renderMonitorStatus(status) {
  const state = document.querySelector('[data-monitor-state]');
  const elapsed = document.querySelector('[data-monitor-elapsed]');
  const apps = document.querySelector('[data-monitor-apps]');
  const stopButton = document.querySelector('[data-stop-monitor]');

  if (state) {
    state.textContent = status.running ? t('monitorRunning') : t('monitorStopped');
  }
  if (elapsed) {
    elapsed.textContent = status.elapsed_seconds ? formatSeconds(status.elapsed_seconds) : '0s';
  }
  if (apps) {
    apps.textContent = status.app_list?.length ? status.app_list.join(', ') : t('noneValue');
  }
  if (stopButton) {
    stopButton.disabled = !status.running;
  }
}

async function refreshMonitorStatus() {
  try {
    const response = await fetch('/rest/status');
    const text = await response.text();
    const status = text ? JSON.parse(text) : { running: false, app_list: [] };
    renderMonitorStatus(status);
  } catch {
    renderMonitorStatus({ running: false, app_list: [] });
  }
}

async function stopMonitoring() {
  const form = document.querySelector('#rest-panel form');
  const button = document.querySelector('[data-stop-monitor]');
  if (!form || !button) return;

  button.disabled = true;
  setStatus(form, t('submitting'));

  try {
    const response = await fetch('/rest/stop', { method: 'POST' });
    if (!response.ok) {
      throw new Error(`${t('requestFailed')}: ${response.status}`);
    }
    setStatus(form, t('stoppedMonitoring'), 'success');
    await refreshMonitorStatus();
  } catch (error) {
    setStatus(form, error.message || t('requestFailed'), 'error');
  }
}

function syncSelectedProcesses() {
  const hiddenInput = document.querySelector('[data-process-value]');
  if (hiddenInput) {
    hiddenInput.value = processState.selected.join(', ');
  }
}

function renderSelectedProcesses() {
  const container = document.querySelector('[data-selected-processes]');
  if (!container) return;

  container.innerHTML = '';

  if (!processState.selected.length) {
    const empty = document.createElement('span');
    empty.className = 'selected-empty';
    empty.textContent = t('selectedAppsEmpty');
    container.appendChild(empty);
    syncSelectedProcesses();
    if (configReady) saveConfig();
    return;
  }

  processState.selected.forEach((name) => {
    const chip = document.createElement('span');
    chip.className = 'process-chip';

    const text = document.createElement('span');
    text.textContent = name;

    const remove = document.createElement('button');
    remove.type = 'button';
    remove.setAttribute('aria-label', `${t('removeApp')} ${name}`);
    remove.textContent = '×';
    remove.addEventListener('click', () => {
      processState.selected = processState.selected.filter((item) => item !== name);
      renderSelectedProcesses();
      renderProcessSuggestions();
    });

    chip.append(text, remove);
    container.appendChild(chip);
  });

  syncSelectedProcesses();
  if (configReady) saveConfig();
}

function processRank(name, query) {
  const lower = name.toLowerCase();
  if (query && lower.startsWith(query)) return 0;
  if (query && lower.includes(query)) return 1;

  const preferredIndex = preferredProcessTerms.findIndex((term) => lower.includes(term));
  if (preferredIndex >= 0) return 10 + preferredIndex;

  if (!/(agent|helper|service|daemon|xpc|crash|login|sync|extension)/i.test(name)) return 50;
  return 100;
}

function addProcess(name) {
  const normalized = name.trim();
  if (!normalized) return;
  if (!processState.selected.some((item) => item.toLowerCase() === normalized.toLowerCase())) {
    processState.selected.push(normalized);
  }
  renderSelectedProcesses();
  renderProcessSuggestions('');
  const input = document.querySelector('.process-search');
  if (input) input.value = '';
}

function renderProcessSuggestions(query = document.querySelector('.process-search')?.value || '') {
  const container = document.querySelector('[data-process-suggestions]');
  if (!container) return;

  container.innerHTML = '';
  container.classList.toggle('is-open', processState.dropdownOpen);
  const normalizedQuery = query.trim().toLowerCase();
  const selected = new Set(processState.selected.map((item) => item.toLowerCase()));
  const matches = processState.all
    .filter((name) => !selected.has(name.toLowerCase()))
    .filter((name) => !normalizedQuery || name.toLowerCase().includes(normalizedQuery))
    .sort((left, right) => {
      const rankDiff = processRank(left, normalizedQuery) - processRank(right, normalizedQuery);
      return rankDiff || left.localeCompare(right);
    })
    .slice(0, normalizedQuery ? 60 : 36);

  if (!matches.length) {
    setProcessPickerMessage(t('noProcessMatches'));
    return;
  }

  matches.forEach((name) => {
    const button = document.createElement('button');
    button.className = 'process-suggestion';
    button.type = 'button';
    button.textContent = name;
    button.addEventListener('click', () => addProcess(name));
    container.appendChild(button);
  });
}

async function loadProcesses() {
  setProcessPickerMessage(t('loadingProcesses'));

  try {
    const response = await fetch('/processes');
    const text = await response.text();
    const payload = text ? JSON.parse(text) : {};

    if (!response.ok) {
      throw new Error(payload.error || `${t('requestFailed')}: ${response.status}`);
    }

    processState.all = Array.isArray(payload.processes) ? payload.processes : [];
    processState.loaded = true;
    processState.dropdownOpen = true;
    renderProcessSuggestions();
  } catch {
    setProcessPickerMessage(t('processLoadFailed'));
  }
}

function getActiveCountLogPath() {
  const activeForm = document.querySelector('#count-panel .count-form.is-active');
  return activeForm?.querySelector('input[name="log_path"]')?.value
    || document.querySelector('#count-panel input[name="log_path"]')?.value
    || '';
}

function renderLogPreview(entries) {
  const list = document.querySelector('[data-log-preview-list]');
  if (!list) return;

  list.innerHTML = '';
  if (!entries.length) {
    const item = document.createElement('li');
    item.textContent = t('logPreviewNoEntries');
    list.appendChild(item);
    return;
  }

  entries.forEach((entry) => {
    const item = document.createElement('li');
    item.textContent = entry;
    list.appendChild(item);
  });
}

async function refreshLogPreview() {
  const logPath = getActiveCountLogPath();
  const button = document.querySelector('[data-log-preview]');

  if (!logPath) {
    renderLogPreview([t('logPreviewEmpty')]);
    return;
  }

  if (button) button.disabled = true;

  try {
    const response = await fetch('/log-preview', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ log_path: logPath, limit: 8 }),
    });
    const text = await response.text();
    const payload = text ? JSON.parse(text) : {};

    if (!response.ok) {
      throw new Error(payload.error || `${t('requestFailed')}: ${response.status}`);
    }

    renderLogPreview(payload.entries || []);
  } catch (error) {
    renderLogPreview([error.message || t('requestFailed')]);
  } finally {
    if (button) button.disabled = false;
  }
}

function showPlotPreview(dataUrl) {
  const preview = document.querySelector('[data-plot-preview]');
  const image = document.querySelector('[data-plot-preview-image]');
  if (!preview || !image) return;

  image.src = dataUrl;
  preview.hidden = false;
}

async function openPathPicker(button) {
  const endpoint = pickerEndpoints[button.dataset.picker];
  const control = button.closest('.path-control');
  const input = control?.querySelector('input');
  const form = button.closest('form');

  if (!endpoint || !input || !form) return;

  button.disabled = true;
  setStatus(form, t('openingPicker'));

  try {
    const response = await fetch(endpoint);
    const text = await response.text();
    let payload = {};
    try {
      payload = text ? JSON.parse(text) : {};
    } catch {
      payload = { error: text };
    }

    if (!response.ok) {
      throw new Error(payload.error || `${t('requestFailed')}: ${response.status}`);
    }

    if (payload.cancelled || !payload.path) {
      setStatus(form, t('pickerCancelled'));
      return;
    }

    input.value = payload.path;
    input.dispatchEvent(new Event('input', { bubbles: true }));
    saveConfig();
    setStatus(form, t('pathSelected'), 'success');
  } catch (error) {
    setStatus(form, error.message || t('requestFailed'), 'error');
  } finally {
    button.disabled = false;
  }
}

function initTabs() {
  const tabs = document.querySelectorAll('.tab');
  const panels = document.querySelectorAll('.panel');

  tabs.forEach((tab) => {
    tab.addEventListener('click', () => {
      tabs.forEach((item) => item.classList.remove('is-active'));
      panels.forEach((panel) => panel.classList.remove('is-active'));
      tab.classList.add('is-active');
      document.getElementById(tab.dataset.panel)?.classList.add('is-active');
    });
  });
}

function initCountModes() {
  const buttons = document.querySelectorAll('[data-count-mode]');
  const forms = document.querySelectorAll('[data-count-form]');

  buttons.forEach((button) => {
    button.addEventListener('click', () => {
      buttons.forEach((item) => item.classList.remove('is-active'));
      forms.forEach((form) => form.classList.remove('is-active'));
      button.classList.add('is-active');
      document.querySelector(`[data-count-form="${button.dataset.countMode}"]`)?.classList.add('is-active');
    });
  });
}

function initTheme() {
  const button = document.getElementById('theme-toggle');
  if (!button) return;

  const preferred = window.matchMedia?.('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  const apply = (theme) => {
    document.documentElement.dataset.theme = theme;
    button.setAttribute('aria-pressed', String(theme === 'dark'));
    button.querySelector('span').textContent = theme === 'dark' ? '☀' : '◐';
  };

  apply(localStorage.getItem(THEME_STORAGE_KEY) || preferred);
  button.addEventListener('click', () => {
    const next = document.documentElement.dataset.theme === 'dark' ? 'light' : 'dark';
    localStorage.setItem(THEME_STORAGE_KEY, next);
    apply(next);
  });
}

function initLanguage() {
  const languageSelect = document.getElementById('language-select');
  translatePage(localStorage.getItem(LANGUAGE_STORAGE_KEY) || DEFAULT_LANGUAGE);

  languageSelect?.addEventListener('change', () => {
    localStorage.setItem(LANGUAGE_STORAGE_KEY, languageSelect.value);
    translatePage(languageSelect.value);
  });
}

function initForms() {
  document.querySelectorAll('form[data-endpoint]').forEach((form) => {
    form.addEventListener('submit', (event) => {
      event.preventDefault();
      submitForm(form);
    });
  });

  document.querySelectorAll('input').forEach((input) => {
    input.addEventListener('change', saveConfig);
  });
}

function initPathPickers() {
  document.querySelectorAll('[data-picker]').forEach((button) => {
    button.addEventListener('click', () => {
      openPathPicker(button);
    });
  });
}

function initProcessPicker() {
  const input = document.querySelector('.process-search');
  const refreshButton = document.querySelector('[data-refresh-processes]');
  if (!input) return;

  renderSelectedProcesses();
  loadProcesses();

  input.addEventListener('input', () => {
    processState.dropdownOpen = true;
    renderProcessSuggestions(input.value);
  });

  input.addEventListener('focus', () => {
    processState.dropdownOpen = true;
    renderProcessSuggestions(input.value);
  });

  input.addEventListener('keydown', (event) => {
    if (event.key !== 'Enter') return;
    event.preventDefault();
    addProcess(input.value);
    input.value = '';
  });

  refreshButton?.addEventListener('click', () => {
    processState.dropdownOpen = true;
    loadProcesses();
  });

  document.addEventListener('click', (event) => {
    if (event.target.closest('[data-process-picker]')) return;
    processState.dropdownOpen = false;
    document.querySelector('[data-process-suggestions]')?.classList.remove('is-open');
  });
}

function initMonitorControls() {
  document.querySelector('[data-stop-monitor]')?.addEventListener('click', stopMonitoring);
  refreshMonitorStatus();
  window.setInterval(refreshMonitorStatus, 5000);
}

function initPreviews() {
  document.querySelector('[data-log-preview]')?.addEventListener('click', refreshLogPreview);
}

initTabs();
initCountModes();
initTheme();
initLanguage();
restoreConfig();
initPathPickers();
initProcessPicker();
initMonitorControls();
initPreviews();
initForms();
